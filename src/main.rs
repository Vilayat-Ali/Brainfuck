#![allow(unused)]

use brainfuck::interpreter::execute_program;
use std::{
    env::{args, Args},
    path::PathBuf,
};

fn main() {
    let args_vec: Vec<String> = args().collect();
    let args_vec = &args_vec[1..];

    if args_vec.len() == 1 {
        // No args passed
        println!("No Args passed!\nTip: Use 'bf run <file path> to execute.");
    } else if args_vec.len() >= 2 {
        match args_vec[0].as_str() {
            "run" => {
                if PathBuf::from(&args_vec[1])
                    .extension()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    == "bf"
                {
                    // valid brainfuck file
                    execute_program(&args_vec[1]);
                } else {
                    println!(
                        "{}",
                        "Invalid source file\nTip: Use source file with filename ending with '.bf'"
                    )
                }
            }
            _ => {
                println!("Hello from _")
            }
        }
    }
}
