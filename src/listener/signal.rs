#![allow(dead_code)]

pub mod signum {
    pub const SHUTDOWN: u8 = 0x80;
    pub const FLUSH: u8 = 0x40;
    pub const RESET_CLOCK: u8 = 0x20;
    pub const STOP_CLOCK: u8 = 0x10;
}

use signum::*;

#[derive(Clone, Copy)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Signal {
    Shutdown   = SHUTDOWN,
    Flush      = FLUSH,
    ResetClock = RESET_CLOCK,
    StopClock  = STOP_CLOCK,
    Nil        = 0x00,
}

impl Default for Signal {
    fn default() -> Self {
        Self::Nil
    }
}

impl Signal {
    pub fn check(&self, signal: u8) -> bool {
        let current = *self as u8;
        signal & current == current
    }
}
