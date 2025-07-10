// Main file which builds the password solver simulator.

mod config;
mod generator;
mod worker;
mod utils;

use generator::Generator;
use utils::{check_match, elapsed, now};

fn main() {
    // Target password to crack (can be anything printable)
    let target_password = "+9#$";
    println!("Starting password cracking simulation...");

    let start_time = now();

    let mut generator = Generator::new();
    let mut attempts = 0;

    loop {
        let guess = generator.next_guess();
        attempts += 1;

        if check_match(&guess, target_password) {
            let duration = elapsed(start_time);
            println!("Password cracked: \"{}\"", guess);
            println!("Attempts: {}", attempts);
            println!("Time taken: {:.2?}", duration);
            break;
        }
        if attempts % 1_000_000_000 == 0 {
            println!("Tried {} ({} attempts)", guess, attempts);
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