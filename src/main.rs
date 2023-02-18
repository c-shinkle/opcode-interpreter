extern crate num_derive;

use opcode_interpreter::opcode::interpreter::interpret;
use std::fs;

fn main() {
    let input = i32::default();
    let mut output = Option::default();
    let codes_string = fs::read_to_string("res/advent_of_code_puzzle").unwrap();
    let result = interpret(&codes_string, input, &mut output);
    match result {
        Ok(codes_string) => {
            println!("codes string");
            println!("{codes_string}");
            println!("output");
            match output {
                Some(value) => println!("{value}"),
                None => println!("None"),
            }
        }
        Err(error) => eprintln!("{error}"),
    }
}
