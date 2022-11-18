use pkit::Result;
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
        .map(Clone::clone)
        .unwrap();

    println!("Listening on device: {:?}", capture);

    let mut capture = Capture::from_device(capture)?
        .timeout(0)
        .immediate_mode(true)
        .open()?;

    loop {
        pkit::analyze_packet(capture.next_packet()?);
    }
}
