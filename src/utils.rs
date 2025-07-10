//Utility Manager for background features such as timer.

use std::time::{Duration, Instant};

pub fn get_charset() -> Vec<char> {
    // Complete charset
    (0x20u8..=0x7Eu8).map(|c| c as char).collect()
}

pub fn check_match(guess: &str, target: &str) -> bool {
    // Password check
    guess == target
}

pub fn now() -> Instant {
    // Get current time
    Instant::now()
}

pub fn elapsed(start: Instant) -> Duration {
    // Calculate elapsed time since start
    start.elapsed()
}