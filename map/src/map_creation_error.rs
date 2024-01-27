use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct MapCreationError{
    message: &'static str
}

impl MapCreationError{
    pub fn new(message: &'static str) -> Self{
        MapCreationError{
            message
        }
    }
}

impl Display for MapCreationError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.message)
    }
}

impl Error for MapCreationError{
    fn description(&self) -> &str {
        self.message
    }

}

impl From<&'static str> for MapCreationError{
    fn from(message: &'static str) -> Self {
        MapCreationError{
            message
        }
    }
}