use anyhow::Result;
use tun::{Configuration};
use tun::platform::Device;

pub fn create_tun() -> Result<Device> {

    let mut config =
        Configuration::default();

    config
        .address("10.0.0.1")
        .netmask("255.255.255.0")
        .mtu(1500)
        .up();

    let dev =
        Device::new(&config)?;

    println!("TUN device created");

    Ok(dev)

}