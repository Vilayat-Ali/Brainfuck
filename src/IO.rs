#![allow(unused)]

use std::io::{prelude::Write, stdin, stdout, Error, ErrorKind, Read, Result, StdoutLock};

pub fn print_to_std_out(payload: &[u8]) -> Result<()> {
    stdout().lock().write_all(payload)?;
    Ok(())
}

pub fn write_to_std_in() -> Result<(u8)> {
    // read only 1 byte of data
    let mut buffer: [u8; 1] = [0; 1]; // value, mode
    stdin().lock().read(&mut buffer)?;
    Ok(buffer[0])
}
