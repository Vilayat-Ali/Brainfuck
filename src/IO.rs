#![allow(unused)]

use std::io::{stdin, stdout, Error, ErrorKind, Read, StdoutLock, Write};

pub fn print_to_std_out(payload: &[u8]) {
    if (payload[0] as char) != ' ' {
        print!("{}", payload[0] as char);
    } else {
        print!("{}", payload[0]);
    }
}

pub fn write_to_std_in() -> std::io::Result<(u8)> {
    // read only 1 byte of data
    let mut buffer: [u8; 1] = [0; 1]; // value, mode
    stdin().lock().read(&mut buffer)?;
    Ok(buffer[0])
}
