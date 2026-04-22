//! RISC-V timer-related functionality

use crate::config::CLOCK_FREQ;
use riscv::register::time;
use sbi_rt::set_timer;
/// The number of ticks per second
const TICKS_PER_SEC: u64 = 100;
#[allow(dead_code)]
/// The number of milliseconds per second
const MSEC_PER_SEC: u64 = 1000;
/// The number of microseconds per second
#[allow(dead_code)]
const MICRO_PER_SEC: u64 = 1_000_000;

/// Get the current time in ticks
pub fn get_time() -> u64 {
    time::read() as u64
}

/// get current time in milliseconds
#[allow(dead_code)]
pub fn get_time_ms() -> u64 {
    time::read() as u64 / CLOCK_FREQ * MSEC_PER_SEC
}

/// get current time in microseconds
#[allow(dead_code)]
pub fn get_time_us() -> u64 {
    time::read() as u64 / CLOCK_FREQ * MICRO_PER_SEC
}

/// Set the next timer interrupt
pub fn set_next_trigger() {
    let current_time = get_time();
    set_timer(current_time + CLOCK_FREQ / TICKS_PER_SEC);
}
