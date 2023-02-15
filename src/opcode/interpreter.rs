use super::errors::OpcodeError::*;
use super::errors::Result;
use super::parse;
use super::stringify;
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
    let mut codes = parse::imperative(codes_string)?;
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

        match operator {
            Operator::Addition => binary_operation(
                &mut i,
                &mut codes,
                first_param_mode,
                second_param_mode,
                |a, b| a + b,
            )?,
            Operator::Multiplication => binary_operation(
                &mut i,
                &mut codes,
                first_param_mode,
                second_param_mode,
                |a, b| a * b,
            )?,
            Operator::ReadInput => {
                bounds_check(i + 1, codes.len())?;
                let destination = usize::try_from(codes[i + 1])?;
                codes[destination] = input;
                i += 2;
            }
            Operator::WriteOutput => {
                bounds_check(i + 1, codes.len())?;
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
    Ok(stringify::precompute_capacity(&codes))
}

fn binary_operation(
    i: &mut usize,
    codes: &mut Vec<i32>,
    first_param_mode: ParameterMode,
    second_param_mode: ParameterMode,
    op: fn(i32, i32) -> i32,
) -> Result<()> {
    bounds_check(*i + 3, codes.len())?;
    let first = match first_param_mode {
        ParameterMode::Position => codes[usize::try_from(codes[*i + 1])?],
        ParameterMode::Immediate => codes[1 + 1],
    };
    let second = match second_param_mode {
        ParameterMode::Position => codes[usize::try_from(codes[*i + 2])?],
        ParameterMode::Immediate => codes[*i + 2],
    };
    let destination = usize::try_from(codes[*i + 3])?;
    codes[destination] = op(first, second);
    *i += 4;
    Ok(())
}

fn bounds_check(index: usize, len: usize) -> Result<()> {
    if index >= len {
        return Err(OutOfBounds(index));
    }
    Ok(())
}
