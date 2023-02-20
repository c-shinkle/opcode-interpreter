// use super::errors::Result;
// use crate::opcode::interpreter::interpret;
// use crate::opcode::parse;
//
// fn compute_max_signal(codes_string: &str) -> Result<i32> {
//     let mut input = 0;
//     let mut output: Option<i32> = Option::default();
//
//     let codes = parse::imperative(codes_string)?;
//
//     let mut amplifier_a = codes.clone();
//     interpret(&mut amplifier_a, input, &mut output)?;
//     input = output.expect("Output to be present");
//
//     let mut amplifier_b = codes.clone();
//     interpret(&mut amplifier_b, input, &mut output)?;
//     input = output.expect("Output to be present");
//
//     let mut amplifier_c = codes.clone();
//     interpret(&mut amplifier_c, input, &mut output)?;
//     input = output.expect("Output to be present");
//
//     let mut amplifier_d = codes.clone();
//     interpret(&mut amplifier_d, input, &mut output)?;
//     input = output.expect("Output to be present");
//
//     let mut amplifier_e = codes;
//     interpret(&mut amplifier_e, input, &mut output)?;
//     input = output.expect("Output to be present");
//
//     Ok(input)
// }
