#![allow(unused)]

use std::io::{stdin, stdout, Error, ErrorKind, Read, StdoutLock, Write};

pub fn print_to_std_out(payload: &[u8]) -> std::io::Result<()> {
    stdout().lock().write(payload)?;
    Ok(())
}

pub fn write_to_std_in() -> std::io::Result<(u8)> {
    // read only 1 byte of data
    let mut buffer: [u8; 1] = [0; 1];
    stdin().lock().read(&mut buffer)?;
    Ok(buffer[0])
}
