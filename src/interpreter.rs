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
    let mut loop_control: (bool, usize) = (false, 0); // loop control variables
    let mut bracket_stack: Vec<u32> = Vec::with_capacity(10000); // using stack for bracket matching

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
                                if memory_block[pointer] == u8::MAX {
                                    println!(
                                        "MemoryError: Integer Overflow at memory index {}",
                                        pointer
                                    );
                                    break;
                                } else {
                                    memory_block[pointer] += 1;
                                }
                            }
                            // Minus Token
                            45 => {
                                if memory_block[pointer] == u8::MIN {
                                    println!(
                                        "MemoryError: Integer Underflow at memory index {}",
                                        pointer
                                    );
                                    break;
                                } else {
                                    memory_block[pointer] -= 1;
                                }
                            }
                            // handling loops
                            91 => {
                                // opening square brackets
                                bracket_stack.push(1);
                            }
                            93 => {
                                // closing square brackets
                                bracket_stack.pop();
                            }
                            // Left Angle Bracket Token
                            60 => {
                                if pointer == 0 {
                                    println!(
                                        "PointerError: Pointer can never have negative values"
                                    );
                                    break;
                                } else {
                                    pointer -= 1;
                                }
                            }
                            // Right Angle Bracket Token
                            62 => {
                                if pointer == 30000 {
                                    println!("PointerError: Max limit of memory block reached");
                                    break;
                                } else {
                                    pointer += 1;
                                }
                            }
                            // Period Token
                            46 => {
                                if let Err(e) = print_to_std_out(&[memory_block[pointer]]) {
                                    println!("IOError: {}", e);
                                    break;
                                };
                            }
                            // Comma Token
                            44 => match write_to_std_in() {
                                Ok(input_val) => {
                                    memory_block[pointer] = input_val;
                                }
                                Err(e) => {
                                    println!("IOError: {}", e);
                                }
                            },
                            // Unrecognised characters are left as comments
                            _ => {}
                        }
                    }
                    if bracket_stack.len() != 0 {
                        println!("BrackMatchError: Loop brackets are not properly closed");
                    }
                    println!(
                        "\n\nFinished: Compiled in {}ms",
                        interpreter_starting_time.elapsed().as_millis()
                    );
                }
                Err(e) => {
                    println!("FileError: {}", e);
                }
            }
        }
        Err(e) => println!("FileError: {}", e),
    };
}
