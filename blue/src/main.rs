// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.

use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};

use std::error::Error;
//use std::thread;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let now = time::Instant::now();
    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    // start scanning for devices
    central.start_scan(ScanFilter::default()).await?;
    // instead of waiting, you can use central.event_receiver() to fetch a channel and
    // be notified of new devices
    time::sleep(Duration::from_millis(560)).await;
    let mut devices: Vec<Peripheral> = vec![];
    // find the device we're interested in

    for p in central.peripherals().await.unwrap() {
        let light = p;

        // connect to the device
        if light.connect().await.is_ok() {
            // discover services and characteristics
            light.discover_services().await?;

            // find the characteristic we want
            //let chars = light.characteristics();
            //
            devices.push(light);
            //
        }
    }

    println!("{:?}", devices);
    println!("{:?}", now.elapsed());
    Ok(())
}
#[allow(dead_code)]
async fn find_device(central: &Adapter) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("Buds"))
        {
            println!("{:?}", p);
            return Some(p);
        }
    }
    None
}
