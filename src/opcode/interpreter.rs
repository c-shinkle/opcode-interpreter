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

pub fn interpret(codes: &mut Vec<i32>, input: i32, output: &mut Option<i32>) -> Result<()> {
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
                codes,
                first_param_mode,
                second_param_mode,
                |a, b| a + b,
            )?,
            Operator::Multiplication => binary_operation(
                &mut i,
                codes,
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
    Ok(())
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
        ParameterMode::Immediate => codes[*i + 1],
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

#[cfg(test)]
mod tests {
    use super::interpret;
    use crate::opcode::parse;
    use std::fs;

    #[test]
    fn day_2_part_1() {
        let codes_string = fs::read_to_string("res/day_2")
            .expect("Should have been able to read the file");
        let mut given = parse::imperative(&codes_string)
            .expect("Should have been able to parse codes from file");
        let mut output = Option::default();

        let actual = interpret(&mut given, 1, &mut output);

        assert!(actual.is_ok());
        assert_eq!(given[0], 7594646);
    }

    #[test]
    fn day_5_part_1() {
        let codes_string = fs::read_to_string("res/day_5")
            .expect("Should have been able to read the file");
        let mut given = parse::imperative(&codes_string)
            .expect("Should have been able to parse codes from file");
        let mut output = Option::default();

        let actual = interpret(&mut given, 1, &mut output);

        assert!(actual.is_ok());
        assert!(output.is_some());
        assert_eq!(output.unwrap(), 11193703);
    }

    #[test]
    fn add_multi_3500() {
        let mut given = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let actual = interpret(&mut given, i32::default(), &mut Option::default());

        assert!(actual.is_ok());
        assert_eq!(given, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn input_1377() {
        let mut given = vec![3, 0, 99];

        let actual = interpret(&mut given, 1337, &mut Option::default());

        assert!(actual.is_ok());
        assert_eq!(given, vec![1337, 0, 99]);
    }

    #[test]
    fn output_1377() {
        let mut given = vec![4, 3, 99, 1337];
        let mut output = None;

        let actual = interpret(&mut given, i32::default(), &mut output);

        assert!(actual.is_ok());
        assert!(output.is_some());
        assert_eq!(output.unwrap(), 1337);
    }

    #[test]
    fn first_param_position() {
        let mut given = vec![102, 3, 4, 4, 33];

        let actual = interpret(&mut given, i32::default(), &mut Option::default());

        assert!(actual.is_ok());
        assert_eq!(given, vec![102, 3, 4, 4, 99]);
    }

    #[test]
    fn second_param_position() {
        let mut given = vec![1002, 4, 3, 4, 33];

        let actual = interpret(&mut given, i32::default(), &mut Option::default());

        assert!(actual.is_ok());
        assert_eq!(given, vec![1002, 4, 3, 4, 99]);
    }
}
