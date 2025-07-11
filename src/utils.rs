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

pub fn is_valid_guess(
    guess: &str,
    required_chars: &[char],
    prefix: &Option<String>,
    suffix: &Option<String>,
) -> bool {
    // Check if guess meets all criteria
    if !required_chars.iter().all(|c| guess.contains(*c)) {
        return false;
    }

    if let Some(pre) = prefix {
        if !guess.starts_with(pre) {
            return false;
        }
    }

    if let Some(suf) = suffix {
        if !guess.ends_with(suf) {
            return false;
        }
    }

    true
}
