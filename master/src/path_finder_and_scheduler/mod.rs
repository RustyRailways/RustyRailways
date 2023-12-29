use std::thread::sleep;
use common_infrastructure::hals::MasterHal;
use crate::high_level_controller::Request;
use crate::low_level_controller::LowLevelController;
use crate::map::views::map_controller_view::MapControllerView;
use crate::map::views::map_factory::MapFactory;
use crate::map::views::map_navigation_view::MapNavigationView;
use anyhow::Result;
use common_infrastructure::devices::Train;
use map_navigation_functions::{path_to_intersection, path_to_move_out_of_the_way, path_to_position, path_to_switch_point};
use crate::map::views::MapVisualizationView;
use crate::path_finder_and_scheduler::map_navigation_functions::{find_path_to_move_out_of_the_way, find_path_to_position};
use crate::path_finder_and_scheduler::map_navigation_functions::path_to_switch_point::find_path_to_switch_point;

mod map_navigation_functions;

#[cfg(test)]
mod tests;

pub struct PathFinderAndScheduler<'a, T: MasterHal> {
    hal: &'a T,
    factory: &'a MapFactory,
    map_controller: MapControllerView<'a, T>,
    map_visualization: MapVisualizationView,
    low_level_controller: LowLevelController<'a, T>,
    requests: Vec<Request>,
}

impl<'a, T: MasterHal> PathFinderAndScheduler<'a, T> {
    pub fn new(hal: &'a T, factory: &'a MapFactory) -> Self{
        Self{
            hal,
            factory,
            map_controller: factory.build_controller_view(hal),
            map_visualization: factory.build_visualization_view(),
            low_level_controller: LowLevelController::new(hal, factory.build_controller_view(hal)),
            requests: Vec::new(),
        }
    }

    /// When a request is added the path finder can either execute it immediately,
    /// this means that the function is blocking, otherwise it can add it to a queue
    /// and execute it when it is possible.
    /// return a list of all the request that were executed.
    pub fn add_request(&mut self, request: Request) -> Result<Vec<Request>>{
        // do not add the same request twice
        if !self.requests.contains(&request){
            self.requests.push(request);
        }

        // execute all the requests that can be executed
        self.execute_requests()
    }

    /// Try to execute a request, return true if the request was executed, false otherwise.
    fn execute_request(&mut self, request: Request)-> Result<bool>{
        let train = request.train_id;
        let destination = request.destination;

        let nv = self.factory.build_navigation_view();
        let path = find_path_to_position(train, destination, &self.map_visualization, &nv, false)?;
        drop(nv);

        // can execute the request without moving any train
        if let Some(path) = path{
            self.low_level_controller.move_train(train, &path)?;
            return Ok(true);
        }

        let nv = self.factory.build_navigation_view();
        let path = find_path_to_position(train, destination, &self.map_visualization, &nv, true)?;

        let path = match path{
            // can't execute the request
            None => return Ok(false),
            // a path was found
            Some(s) => s
        };

        let other_train = match train {
            Train::T1 => Train::T2,
            Train::T2 => Train::T1
        };

        let path_to_move_out_of_the_way = find_path_to_move_out_of_the_way(other_train, &path, &self.map_visualization, &nv)?;
        drop(nv);

        // the other train can move out of the way easily
        if let Some(path_to_move_out_of_the_way) = path_to_move_out_of_the_way{
            self.low_level_controller.move_train(other_train, &path_to_move_out_of_the_way)?;
            self.low_level_controller.move_train(train, &path)?;
            return Ok(true);
        }

        let nv = self.factory.build_navigation_view();
        let path_to_switch_point = find_path_to_switch_point(train, destination, &self.map_visualization, &nv)?;

        let path_to_switch_point = match path_to_switch_point {
            // can't execute the request
            None => return Ok(false),
            Some(path_to_switch_point) => {path_to_switch_point}
        };

        drop(nv);

        self.low_level_controller.move_train(train, &path_to_switch_point)?;

        // now the train is in a switch, the request should be able to be executed
        // with this recursive call, since now the second train can move out of the way
        let r = self.execute_request(request)?;
        assert!(r);
        return Ok(true);
    }

    /// Try to execute all the requests in the queue, remove the ones that are executed.
    /// May leave some requests in the queue if they cannot be executed.
    /// return a list of all the request that were executed.
    fn execute_requests(&mut self) -> Result<Vec<Request>>{

        let mut executed_requests = Vec::new();

        loop {
            let mut success = false;
            let mut executed_requests_partial = Vec::new();

            for request_id in 0..self.requests.len(){
                let request = self.requests[request_id];
                if self.execute_request(request)?{
                    success = true;
                    executed_requests_partial.push(request);
                }
            }

            if success{
                self.requests = self.requests.iter()
                    .filter(|request| !executed_requests_partial.contains(request))
                    .map(|request| *request)
                    .collect();

                executed_requests.append(&mut executed_requests_partial);
            }else{
                break;
            }
        }
        return Ok(executed_requests);
    }
}