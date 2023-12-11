use crate::devices::Device;
use anyhow::{Ok,Result};
use serde_json;

#[test]
fn test()->Result<()>{
    let d = Device::Train(devices::Train::T1);
    let d_str = serde_json::to_string(&d)?;
    println!("{d_str}");
    let d2 = serde_json::from_str::<Device>(&d_str)?;
    assert_eq!(d,d2);
    Ok(())
}