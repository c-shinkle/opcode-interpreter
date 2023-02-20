extern crate num_derive;

use std::fs::read_to_string;

use opcode_interpreter::opcode::interpreter::interpret;
use opcode_interpreter::opcode::parse;

fn main() {
    //read file
    let result_codes_string = read_to_string("res/day_2");
    // let result_codes_string = read_to_string("res/day_5");
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
    let template_codes = result_codes.unwrap();
    let mut output = Option::default();
    for noun in 0..100 {
        for verb in 0..100 {
            let mut codes = template_codes.clone();
            codes[1] = noun;
            codes[2] = verb;
            if let Err(error) = interpret(&mut codes, 1, &mut output) {
                eprintln!("{error}");
                continue;
            }
            //print results
            if codes[0] == 19690720 {
                println!("I found it! The noun is {noun} and the verb is {verb}!");
                return;
            }
        }
    }
    println!("No solution was found 😕");
}
