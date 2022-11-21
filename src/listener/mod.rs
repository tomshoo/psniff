#![allow(dead_code, unused)]

use std::collections::VecDeque;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

use pcap::{Active, Capture, Packet, PacketHeader};

use crate::Result;

pub struct BufferedListener {
    capacity: usize,
}

impl BufferedListener {
    pub fn new() -> Self {
        Self { capacity: 5 }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { capacity }
    }

    pub fn listener(&self, mut capture: Capture<Active>) -> (JoinHandle<Result<()>>, Sender<bool>) {
        let (tx, rx) = mpsc::channel();
        let buffer = VecDeque::with_capacity(self.capacity);
        (thread::spawn(move || Self::worker(capture, buffer, rx)), tx)
    }

    fn worker(
        mut capture: Capture<Active>,
        mut buf: VecDeque<(PacketHeader, Vec<u8>)>,
        receiver: Receiver<bool>,
    ) -> Result<()> {
        loop {
            match capture.next_packet() {
                Ok(packet) => {
                    if buf.len() < buf.capacity() {
                        let header = *packet.header;
                        let data = packet.data.into();
                        buf.push_back((header, data));
                    } else {
                        while let Some(tup) = buf.pop_back() {
                            let packet = Packet {
                                header: &tup.0,
                                data: &tup.1,
                            };
                            crate::analyze_packet(packet);
                        }
                        crate::analyze_packet(packet)
                    }
                }
                Err(pcap::Error::TimeoutExpired) => continue,
                Err(e) => Err(e)?,
            }
        }
    }
}

impl Default for BufferedListener {
    fn default() -> Self {
        Self::new()
    }
}

