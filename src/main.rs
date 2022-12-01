use pkit::listener::signal::signum::{FLUSH, SHUTDOWN};
use pkit::listener::BufferedListener;
use pkit::Result;
use std::sync::{Arc, Mutex};

use pcap::{Capture, Device};
use tokio::signal::ctrl_c;

async fn main_() -> Result<()> {
    let mut args = std::env::args();

    let nic = args
        .nth(1)
        .ok_or_else(|| "Pleas provide an NIC".to_string())?;

    let device = Device::list()?
        .into_iter()
        .find(|dev| dev.name == nic)
        .ok_or(format!("Not a valid device name: {}", nic))?;

    let capture = Capture::from_device(device)?
        .timeout(5000)
        .immediate_mode(true)
        .open()?;

    let (listener_handle, tx) = BufferedListener::with_capacity(10).listener(capture);
    let ctrlc_handle = tokio::spawn(async move { ctrl_c().await });

    let txptr = Arc::from(Mutex::from(tx));
    {
        let txptr = Arc::clone(&txptr);
        tokio::select! {
            _ = ctrlc_handle => {
                txptr.lock().map(|sender|sender.send(SHUTDOWN|FLUSH)).unwrap().unwrap();
                println!("CTRLC received");
            }
            obj = listener_handle => {
                obj.unwrap()?
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    return tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(main_());
}
