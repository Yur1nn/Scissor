//! Event logging for Scissor
//! Logs actions, errors, and admin actions.

use std::fs::{OpenOptions};
use std::io::Write;
use chrono::Local;

pub struct Logger;

impl Logger {
    pub fn log(msg: &str) {
        let now = Local::now();
        let line = format!("[{}] {}\n", now.format("%Y-%m-%d %H:%M:%S"), msg);
        print!("{}", line);
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("scissor.log") {
            let _ = file.write_all(line.as_bytes());
        }
    }
}
