#![allow(unused)]

use std::io::{prelude::Write, stdin, stdout, Error, ErrorKind, Read, Result, StdoutLock};

pub fn print_to_std_out(payload: &[u8]) {
    match stdout().lock().write_all(payload) {
        Ok(()) => (),
        Err(e) => println!("IOError: {}", e),
    }
}

pub fn write_to_std_in() -> std::io::Result<(u8)> {
    // read only 1 byte of data
    let mut buffer: [u8; 1] = [0; 1]; // value, mode
    stdin().lock().read(&mut buffer)?;
    Ok(buffer[0])
}
