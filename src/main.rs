// Main file which builds the password solver simulator.

mod generator;
mod utils;
mod config;
mod worker;

use config::Config;
use generator::Generator;
use utils::{check_match, elapsed, is_valid_guess, now};

fn main() {
    let target_password = "D3@r";
    let config = Config::default();

    let start_time = now();
    let mut generator = Generator::new(config);
    let mut attempts = 0;
    println!("🔍 Starting password cracking simulation...");

    while let Some(guess) = generator.next_guess() {
        attempts += 1;

        if !is_valid_guess(
            &guess,
            &generator.config.required_chars,
            &generator.config.prefix,
            &generator.config.suffix,
        ) {
            continue;
        }

        if check_match(&guess, target_password) {
            let duration = elapsed(start_time);
            println!("✅ Password cracked: \"{}\"", guess);
            println!("🔁 Attempts: {}", attempts);
            println!("⏱️  Time taken: {:.2?}", duration);
            break;
        }
        
    }
}


/*
======================================================
Password Cracker Simulator – Educational Use Only
------------------------------------------------------
This tool is a safe, offline simulation designed to
explore password generation algorithms, filtering,
and multithreaded processing in Rust.

⚠️ DO NOT use this code to attempt access to real
accounts, networks, or user data. This is strictly
for academic and personal learning.

Author: Daevik Jain
======================================================
*/