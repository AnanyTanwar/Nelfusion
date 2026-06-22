use super::commands::handle_command;
use std::io::{self, BufRead};

pub fn run() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line == "quit" {
            break;
        }

        handle_command(line);
    }
}
