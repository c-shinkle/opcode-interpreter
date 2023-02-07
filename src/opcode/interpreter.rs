use super::errors::OpcodeError::*;
use super::errors::Result;
use num_traits::FromPrimitive;

#[derive(FromPrimitive, PartialEq)]
pub enum Operator {
    Addition = 1,
    Multiplication = 2,
    ReadInput = 3,
    WriteOutput = 4,
    Terminate = 99,
}

pub fn interpret(codes_string: &str, input: i32, output: &mut Option<i32>) -> Result<String> {
    //parse input
    let mut codes = Vec::new();
    for token in codes_string.split(',') {
        codes.push(token.parse::<i32>()?);
    }
    //executing Opcode
    let mut i = 0;
    while i < codes.len() {
        let operator = FromPrimitive::from_i32(codes[i]).ok_or(BadOperator(codes[i]))?;

        match operator {
            Operator::Addition => {
                if i + 3 >= codes.len() {
                    return Err(OutOfBounds(i));
                }
                let first_pos = usize::try_from(codes[i + 1])?;
                let second_pos = usize::try_from(codes[i + 2])?;
                let destination = usize::try_from(codes[i + 3])?;
                codes[destination] = codes[first_pos] + codes[second_pos];
                i += 4;
            }
            Operator::Multiplication => {
                if i + 3 >= codes.len() {
                    return Err(OutOfBounds(i));
                }
                let first_pos = usize::try_from(codes[i + 1])?;
                let second_pos = usize::try_from(codes[i + 2])?;
                let destination = usize::try_from(codes[i + 3])?;
                codes[destination] = codes[first_pos] * codes[second_pos];
                i += 4;
            }
            Operator::ReadInput => {
                if i + 1 >= codes.len() {
                    return Err(OutOfBounds(i));
                }
                let destination = usize::try_from(codes[i + 1])?;
                codes[destination] = input;
                i += 2;
            }
            Operator::WriteOutput => {
                if i + 1 >= codes.len() {
                    return Err(OutOfBounds(i));
                }
                let pos = usize::try_from(codes[i + 1])?;
                *output = Some(codes[pos]);
                i += 2;
            }
            Operator::Terminate => break,
        }
    }
    //stringify output
    let mut chars = codes
        .into_iter()
        .map(|code| code.to_string() + ",")
        .collect::<String>();
    chars.pop();
    Ok(chars)
}
