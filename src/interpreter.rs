#![allow(unused)]

use crate::IO::{print_to_std_out, write_to_std_in};
use std::{
    fs::{read, File},
    io::Read,
    path::PathBuf,
    time::Instant,
};

const MAX_BUFFER_SIZE: usize = 10;

pub fn execute_program(file_path: &String) {
    // memory array
    let mut memory_block: [u8; 30000] = [0; 30000];
    let mut pointer: usize = 0;
    let mut loop_control: (bool, u8) = (false, 0); // loop control variables
    match File::open(PathBuf::from(file_path)) {
        Ok(mut source) => {
            let mut buffer: [u8; MAX_BUFFER_SIZE] = [0; MAX_BUFFER_SIZE];
            let mut interpreter_starting_time: Instant = Instant::now();
            match source.read(&mut buffer) {
                Ok(status) => {
                    // parsing tokens
                    for token in buffer.into_iter() {
                        match token {
                            // Plus Token
                            43 => {
                                memory_block[pointer] += 1;
                            }
                            // Minus Token
                            45 => {
                                memory_block[pointer] -= 1;
                            }
                            // Left Square Bracket Token
                            91 => {
                                // loop body starts
                                loop_control.0 = true;
                                loop_control.1 = memory_block[pointer];
                            }
                            // Right Square Bracket Token
                            93 => {
                                // loop body ends
                                loop_control.0 = false;
                                loop_control.1 = memory_block[pointer];
                            }
                            // Left Angle Bracket Token
                            60 => {
                                pointer -= 1;
                            }
                            // Right Angle Bracket Token
                            62 => {
                                pointer += 1;
                            }
                            // Period Token
                            46 => {
                                print_to_std_out(&[memory_block[pointer]]);
                            }
                            // Comma Token
                            44 => {
                                if let Ok(input_val) = write_to_std_in() {
                                    memory_block[pointer] = input_val;
                                }
                            }
                            // Unrecognised characters are left as comments
                            _ => {}
                        }
                    }
                    println!(
                        "\nFinished: Compiled in {}ms",
                        interpreter_starting_time.elapsed().as_millis()
                    );
                }
                Err(e) => panic!("{}", e),
            }
            println!("{:?}", buffer);
        }
        Err(e) => panic!("{}", e),
    };
}
