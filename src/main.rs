extern crate num_derive;

use opcode_interpreter::opcode::amplifier::multi_threaded_compute_max_signal;

fn main() {
    let codes_string = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    match multi_threaded_compute_max_signal(codes_string) {
        Ok(signal) => println!("signal\n{signal}"),
        Err(opcode_error) => println!("{opcode_error}"),
    }
}
