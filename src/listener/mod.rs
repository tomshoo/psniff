pub mod signal;

use crate::{analyze_packet, Result};
use pcap::{Active, Capture, Packet};
use std::collections::VecDeque;
use std::sync::mpsc::{self, Receiver, Sender};
use tokio::task::JoinHandle;

use self::signal::Signal;

pub struct BufferedListener {
    capacity: usize,
}

fn flush_queue<T, F, R>(queue: &mut VecDeque<T>, call: F) -> Option<Vec<R>>
where
    R: 'static,
    F: Fn(T) -> R,
{
    use std::any::TypeId;

    let mut results = (TypeId::of::<R>() == TypeId::of::<()>()).then(Vec::new);

    while let Some(object) = queue.pop_back() {
        let r = call(object);
        if let Some(vec) = results.as_mut() {
            vec.push(r)
        }
    }

    results
}

impl BufferedListener {
    pub fn new() -> Self {
        Self { capacity: 5 }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { capacity }
    }

    pub fn capacity(&mut self, capacity: usize) {
        self.capacity = capacity;
    }

    pub fn listener(self, capture: Capture<Active>) -> (JoinHandle<Result<()>>, Sender<u8>) {
        let (tx, rx) = mpsc::channel();
        let worker = async move { self.worker(capture, rx).await };
        (tokio::spawn(worker), tx)
    }

    async fn worker(&self, mut capture: Capture<Active>, receiver: Receiver<u8>) -> Result<()> {
        println!("Spawned worker");
        let mut buf = VecDeque::with_capacity(self.capacity);
        loop {
            let packet = match capture.next_packet() {
                Err(pcap::Error::TimeoutExpired) => continue,
                res => res?,
            };

            if buf.len() < self.capacity {
                buf.push_back((*packet.header, packet.data.to_vec()));
                continue;
            }

            flush_queue(&mut buf, |(ref header, ref data)| {
                analyze_packet(Packet { header, data })
            });

            analyze_packet(packet);

            let bitmask = receiver.try_recv().unwrap_or_default();

            if Signal::Flush.check(bitmask) {
                flush_queue(&mut buf, |(ref header, ref data)| {
                    analyze_packet(Packet { header, data })
                });
            }

            if Signal::Shutdown.check(bitmask) {
                return Ok(());
            }
        }
    }
}

impl Default for BufferedListener {
    fn default() -> Self {
        Self::new()
    }
}
