extern crate num_derive;

use opcode_interpreter::opcode::interpreter::interpret;
use opcode_interpreter::opcode::parse;
use opcode_interpreter::opcode::stringify;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut codes = parse::imperative(&fs::read_to_string("res/day_5")?)?;
    let mut output = None;
    interpret(&mut codes, vec![5], &mut output)?;

    println!("codes string");
    println!("{}", stringify::precompute_capacity(&codes));
    println!("output");
    println!("{output:?}");
    Ok(())
}
