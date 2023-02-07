#[macro_use]
extern crate num_derive;
pub mod opcode;

use opcode::interpreter::interpret;

const TEST_0: &str = "1,9,10,3,2,3,11,0,99,30,40,50";
const TEST_1: &str = "3,0,4,0,99";

fn main() {
    let input = 1337;
    let mut output = Option::default();
    let result = interpret(TEST_0, input, &mut output);
    match result {
        Ok(codes_string) => {
            println!("codes string");
            println!("{codes_string}");
            println!("output");
            println!("{output:?}");
        },
        Err(error) => eprintln!("{error}"),
    }
}
