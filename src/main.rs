#![allow(unused)]

use brainfuck::interpreter::execute_program;
use std::{
    env::{args, Args},
    path::PathBuf,
};

fn main() {
    let args_vec: Vec<String> = args().collect();

    if args_vec.len() == 0 {
        // No args passed
        println!("No Args passed!\nTip: Use 'bf run <file path> to execute.");
    } else if args_vec.len() < 2 {
        match args_vec[0].as_str() {
            "run" => {
                // execute
                execute_program(&args_vec[1]);
            }
            _ => {}
        }
    }
}
