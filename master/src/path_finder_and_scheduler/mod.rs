use common_infrastructure::hals::MasterHal;
use crate::high_level_controller::Request;
use crate::low_level_controller::LowLevelController;
use crate::map::views::map_controller_view::MapControllerView;
use crate::map::views::map_factory::MapFactory;
use crate::map::views::map_navigation_view::MapNavigationView;
use anyhow::Result;


mod map_navigation_functions;

pub struct PathFinderAndScheduler<'a, T: MasterHal> {
    hal: &'a T,
    factory: &'a MapFactory,
    map_controller: MapControllerView<'a, T>,
    low_level_controller: LowLevelController<'a, T>,
    requests: Vec<Request>,
}

impl<'a, T: MasterHal> PathFinderAndScheduler<'a, T> {
    pub fn new(hal: &'a T, factory: &'a MapFactory) -> Self{
        Self{
            hal,
            factory,
            map_controller: factory.build_controller_view(hal),
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
        todo!()
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