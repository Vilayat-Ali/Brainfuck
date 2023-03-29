use std::io::*;

fn main() {
    let mut buffer: [u8; 8] = [0; 8];
    stdin().lock().read(&mut buffer).unwrap();
    stdout().lock().write(&buffer).unwrap();
}
