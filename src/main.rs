use pkit::{listener::BufferedListener, Result};
use std::io::{BufRead, Write};

use pcap::{Capture, Device};

fn main() -> Result<()> {
    let devlist = Device::list()?;

    for (i, d) in devlist.iter().enumerate() {
        println!("{:3}: {}", i, d.name);
    }

    print!("Enter device index: ");
    std::io::stdout().flush()?;
    let inbuf = &mut String::new();
    std::io::stdin().lock().read_line(inbuf)?;

    let capture = devlist
        .get(inbuf.trim().parse::<usize>().unwrap())
        .map(Device::clone)
        .unwrap();

    println!("Listening on device: {:?}", capture);

    let capture = Capture::from_device(capture)?
        .timeout(0)
        .immediate_mode(true)
        .open()?;

    let (listener, tx) = BufferedListener::with_capacity(8).listener(capture);

    tx.send(false).unwrap();

    dbg!(listener.join()).unwrap()

    // loop {
    //     match capture.next_packet() {
    //         Ok(packet) => pkit::analyze_packet(packet),
    //         Err(pcap::Error::TimeoutExpired) => (),
    //         Err(e) => Err(e)?,
    //     }
    // }
    // Ok(())
}
