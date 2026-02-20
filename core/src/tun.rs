use anyhow::Result;
use tun::Configuration;
use tun::platform::Device;

pub fn create_tun() -> Result<Device> {

    let mut config = Configuration::default();

    config.address("10.0.0.1");
    config.netmask("255.255.255.0");
    config.mtu(1500);
    config.up();

    let dev = Device::new(&config).unwrap();

    println!("TUN device created");

    Ok(dev)
}