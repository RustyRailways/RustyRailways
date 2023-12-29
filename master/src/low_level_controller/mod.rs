use common_infrastructure::devices::Train;
use common_infrastructure::hals::MasterHal;
use common_infrastructure::Position;
use crate::map::views::map_controller_view::MapControllerView;
use anyhow::Result;
pub struct LowLevelController<'a, T: MasterHal> {
    hal: &'a T,
    map_controller: MapControllerView<'a, T>,
}

impl<'a,T:MasterHal> LowLevelController<'a,T> {
    pub fn new(hal: &'a T, map_controller: MapControllerView<'a,T>) -> Self{
        Self{
            hal,
            map_controller
        }
    }

    pub fn move_train(&mut self, train: Train, stations: &[Position]) -> Result<()>{
        println!("moving train {:?} to {:?} path: {:?}", train, stations.last().unwrap(),stations);
        unsafe {
            // nota per federico: questa non é l'implementazione corretta, mi serviva soltanto
            // per testare lo scheduler... cavala pure... Non servirá usare unsafe nell' implementazione
            // finale
            self.map_controller.move_train_unchecked(train, *stations.last().unwrap())?;
        }
        Ok(())
    }
}