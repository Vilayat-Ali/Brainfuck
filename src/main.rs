#![allow(unused)]

use brainfuck::IO::{print_to_std_out, write_to_std_in};
use clap::{Args, Parser, Subcommand};
use std::{
    fs::{read, File},
    io::Read,
    path::PathBuf,
};

const MAX_BUFFER_SIZE: usize = 50000;

#[derive(Args)]
pub struct RunProgramArgs {
    pub file_path: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Run(RunProgramArgs),
    Lint,
}

fn main() {
    let interpreter: Cli = Cli::parse();

    match &interpreter.command {
        Commands::Run(_ctx) => {
            // memory array
            let mut memory_block: [u8; 30000] = [0; 30000];
            let mut pointer: usize = 0;
            match File::open(PathBuf::from(&_ctx.file_path)) {
                Ok(mut source) => {
                    let mut buffer: [u8; MAX_BUFFER_SIZE] = [0; MAX_BUFFER_SIZE];
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
                                    91 => {}
                                    // Right Square Bracket Token
                                    93 => {}
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
                                        print_to_std_out(&[memory_block[pointer]]).unwrap();
                                    }
                                    // Comma Token
                                    44 => {
                                        match write_to_std_in() {
                                            Ok(input_val) => {
                                                memory_block[pointer] = input_val;
                                            }
                                            Err(_) => println!("I/O Error: Cannot read from stdin"),
                                        };
                                    }
                                    // Unrecognised characters are left as comments
                                    _ => {}
                                }
                            }
                        }
                        Err(e) => panic!("{}", e),
                    }
                }
                Err(e) => panic!("{}", e),
            };
        }

        Commands::Lint => {}
    }
}
