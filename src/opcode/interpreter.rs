use super::errors::OpcodeError::*;
use super::errors::Result;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
pub enum Operator {
    Addition = 1,
    Multiplication = 2,
    ReadInput = 3,
    WriteOutput = 4,
    Terminate = 99,
}

#[derive(FromPrimitive)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

pub fn interpret(codes_string: &str, input: i32, output: &mut Option<i32>) -> Result<String> {
    //parse input
    let mut codes = Vec::new();
    for token in codes_string.split(',') {
        codes.push(token.parse::<i32>()?);
    }
    //executing Opcode
    let mut i = 0;
    loop {
        let opcode = codes[i];

        let first_param = (opcode / 100) % 10;
        let first_param_mode =
            FromPrimitive::from_i32(first_param).ok_or(BadParameterMode(first_param))?;

        let second_param = (opcode / 1000) % 10;
        let second_param_mode =
            FromPrimitive::from_i32(second_param).ok_or(BadParameterMode(second_param))?;

        let operator_i32 = opcode % 100;
        let operator = FromPrimitive::from_i32(operator_i32).ok_or(BadOperator(operator_i32))?;

        let mut binary_operation = |op: fn(i32, i32) -> i32| {
            if i + 3 >= codes.len() {
                return Err(OutOfBounds(i + 3));
            }
            let first = match first_param_mode {
                ParameterMode::Position => codes[usize::try_from(codes[i + 1])?],
                ParameterMode::Immediate => codes[1 + 1],
            };
            let second = match second_param_mode {
                ParameterMode::Position => codes[usize::try_from(codes[i + 2])?],
                ParameterMode::Immediate => codes[i + 2],
            };
            let destination = usize::try_from(codes[i + 3])?;
            codes[destination] = op(first, second);
            i += 4;
            Ok(())
        };

        match operator {
            Operator::Addition => binary_operation(|a, b| a + b)?,
            Operator::Multiplication => binary_operation(|a, b| a * b)?,
            Operator::ReadInput => {
                if i + 1 >= codes.len() {
                    return Err(OutOfBounds(i + 1));
                }
                let destination = usize::try_from(codes[i + 1])?;
                codes[destination] = input;
                i += 2;
            }
            Operator::WriteOutput => {
                if i + 1 >= codes.len() {
                    return Err(OutOfBounds(i + 1));
                }
                *output = Some(codes[usize::try_from(codes[i + 1])?]);
                i += 2;
            }
            Operator::Terminate => break,
        }
        if i >= codes.len() {
            return Err(OutOfBounds(i));
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
