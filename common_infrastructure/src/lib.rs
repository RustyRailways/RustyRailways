// module that keeps the ips of the object as well as the trait to get the ip from a strict
mod urls;
pub use urls::HasUrl;
pub use urls::URL_MASTER;
pub use urls::IP_MASTER;
pub use urls::MASTER_PORT_CONTROLLER;
pub use urls::MASTER_PORT_VISUALIZER;

#[macro_use]
extern crate num_derive;

pub mod devices;

mod positions;
pub use positions::Position;

pub mod wifi_credentials;

pub mod messages;

// hardware absraction layers for the varius components;
pub mod hals;

#[cfg(test)]
mod tests;