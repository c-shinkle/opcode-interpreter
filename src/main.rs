extern crate num_derive;

use std::fs::read_to_string;

use opcode_interpreter::opcode::interpreter::interpret;
use opcode_interpreter::opcode::{parse, stringify};

fn main() {
    //read file
    let result_codes_string = read_to_string("res/advent_of_code_puzzle");
    if let Err(error) = result_codes_string {
        eprintln!("{error}");
        return;
    }
    //parse string
    let codes_string = result_codes_string.unwrap();
    let result_codes = parse::imperative(&codes_string);
    if let Err(error) = result_codes {
        eprintln!("{error}");
        return;
    }
    //interpret codes
    let mut codes = result_codes.unwrap();
    let input = i32::default();
    let mut output = Option::default();
    if let Err(error) = interpret(&mut codes, input, &mut output) {
        eprintln!("{error}");
        return;
    }
    //print results
    let codes_string = stringify::precompute_capacity(&codes);
    println!("codes string");
    println!("{codes_string}");
    println!("output");
    match output {
        Some(value) => println!("{value}"),
        None => println!("None"),
    }
}
