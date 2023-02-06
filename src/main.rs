#[macro_use]
extern crate num_derive;
pub mod opcode;

use opcode::interpreter::interpret;

fn main() {
    let result = interpret("1,9,10,3,2,3,11,0,99,30,40,50");
    match result {
        Ok(output) => println!("{output}"),
        Err(error) => eprintln!("{error}"),
    }
}
