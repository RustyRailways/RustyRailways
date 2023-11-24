// module that keeps the ips of the object as well as the trait to get the ip from a strict
mod ip_addresses;
pub use ip_addresses::HasIpAddress;

pub mod devices;

mod positions;
pub use positions::Position;

pub mod messages;

// hardware absraction layers for the varius components;
pub mod hals;

mod tests;