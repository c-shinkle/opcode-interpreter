use super::errors::OpcodeError::*;
use super::errors::Result;
use num_traits::FromPrimitive;
use Operator::*;

#[derive(FromPrimitive)]
pub enum Operator {
    Addition = 1,
    Multiplication = 2,
    ReadInput = 3,
    WriteOutput = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equal = 8,
    Terminate = 99,
}

#[derive(FromPrimitive)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

pub fn interpret(
    codes: &mut Vec<i32>,
    mut input: Vec<i32>,
    output: &mut Option<i32>,
) -> Result<()> {
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
            Addition => arithmetic_operation(
                &mut i,
                codes,
                first_param_mode,
                second_param_mode,
                |a, b| a + b,
            )?,
            Multiplication => arithmetic_operation(
                &mut i,
                codes,
                first_param_mode,
                second_param_mode,
                |a, b| a * b,
            )?,
            ReadInput => {
                let first_code = *codes.get(i + 1).ok_or(OutOfBounds(i + 1))?;
                let destination = usize::try_from(first_code)?;
                codes[destination] = input.pop().ok_or(MissingInput)?;
                i += 2;
            }
            WriteOutput => {
                let first_code = *codes.get(i + 1).ok_or(OutOfBounds(i + 1))?;
                *output = Some(match first_param_mode {
                    ParameterMode::Position => codes[usize::try_from(first_code)?],
                    ParameterMode::Immediate => first_code,
                });
                i += 2;
            }
            JumpIfTrue => {
                jump(&mut i, codes, first_param_mode, second_param_mode, |a| {
                    a != 0
                })?;
            }
            JumpIfFalse => {
                jump(&mut i, codes, first_param_mode, second_param_mode, |a| {
                    a == 0
                })?;
            }
            LessThan => arithmetic_operation(
                &mut i,
                codes,
                first_param_mode,
                second_param_mode,
                |a, b| (a < b) as i32,
            )?,
            Equal => arithmetic_operation(
                &mut i,
                codes,
                first_param_mode,
                second_param_mode,
                |a, b| (a == b) as i32,
            )?,
            Terminate => break,
        }
        if i >= codes.len() {
            return Err(OutOfBounds(i));
        }
    }
    Ok(())
}

fn arithmetic_operation(
    i: &mut usize,
    codes: &mut [i32],
    first_mode: ParameterMode,
    second_mode: ParameterMode,
    op: fn(i32, i32) -> i32,
) -> Result<()> {
    let first_code = *codes.get(*i + 1).ok_or(OutOfBounds(*i + 1))?;
    let first_param = match first_mode {
        ParameterMode::Position => codes[usize::try_from(first_code)?],
        ParameterMode::Immediate => first_code,
    };
    let second_code = *codes.get(*i + 2).ok_or(OutOfBounds(*i + 2))?;
    let second_param = match second_mode {
        ParameterMode::Position => codes[usize::try_from(second_code)?],
        ParameterMode::Immediate => second_code,
    };
    let destination = usize::try_from(*codes.get(*i + 3).ok_or(OutOfBounds(*i + 3))?)?;
    codes[destination] = op(first_param, second_param);
    *i += 4;
    Ok(())
}

fn jump(
    i: &mut usize,
    codes: &[i32],
    first_param_mode: ParameterMode,
    second_param_mode: ParameterMode,
    op: fn(i32) -> bool,
) -> Result<()> {
    let first_code = *codes.get(*i + 1).ok_or(OutOfBounds(*i + 1))?;
    let first_param = match first_param_mode {
        ParameterMode::Position => codes[usize::try_from(first_code)?],
        ParameterMode::Immediate => first_code,
    };
    if op(first_param) {
        let second_code = *codes.get(*i + 2).ok_or(OutOfBounds(*i + 2))?;
        let instruction_pointer = match second_param_mode {
            ParameterMode::Position => codes[usize::try_from(second_code)?],
            ParameterMode::Immediate => second_code,
        };
        *i = usize::try_from(instruction_pointer)?;
    } else {
        *i += 3;
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
        let codes_string =
            fs::read_to_string("res/day_2").expect("Should have been able to read the file");
        let mut given = parse::imperative(&codes_string)
            .expect("Should have been able to parse codes from file");
        let mut output = Option::default();

        let actual = interpret(&mut given, vec![1], &mut output);

        assert!(actual.is_ok());
        assert_eq!(given[0], 7594646);
    }

    #[test]
    fn day_5_part_1() {
        let codes_string =
            fs::read_to_string("res/day_5").expect("Should have been able to read the file");
        let mut given = parse::imperative(&codes_string)
            .expect("Should have been able to parse codes from file");
        let mut output = Option::default();

        let actual = interpret(&mut given, vec![1], &mut output);

        assert!(actual.is_ok());
        assert!(output.is_some());
        assert_eq!(output.unwrap(), 11193703);
    }

    #[test]
    fn day_5_part_2() {
        let codes_string =
            fs::read_to_string("res/day_5").expect("Should have been able to read the file");
        let mut given = parse::imperative(&codes_string)
            .expect("Should have been able to parse codes from file");
        let mut output = Option::default();

        let actual = interpret(&mut given, vec![5], &mut output);

        assert!(actual.is_ok());
        assert!(output.is_some());
        assert_eq!(output.unwrap(), 12410607);
    }

    #[test]
    fn add_multi_3500() {
        let mut given = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let actual = interpret(&mut given, vec![i32::default()], &mut Option::default());

        assert!(actual.is_ok());
        assert_eq!(given, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn input_1377() {
        let mut given = vec![3, 0, 99];

        let actual = interpret(&mut given, vec![1337], &mut Option::default());

        assert!(actual.is_ok());
        assert_eq!(given, vec![1337, 0, 99]);
    }

    #[test]
    fn output_1377() {
        let mut given = vec![4, 3, 99, 1337];
        let mut output = None;

        let actual = interpret(&mut given, vec![i32::default()], &mut output);

        assert!(actual.is_ok());
        assert!(output.is_some());
        assert_eq!(output.unwrap(), 1337);
    }

    #[test]
    fn first_param_position() {
        let mut given = vec![102, 3, 4, 4, 33];

        let actual = interpret(&mut given, vec![i32::default()], &mut Option::default());

        assert!(actual.is_ok());
        assert_eq!(given, vec![102, 3, 4, 4, 99]);
    }

    #[test]
    fn second_param_position() {
        let mut given = vec![1002, 4, 3, 4, 33];

        let actual = interpret(&mut given, vec![i32::default()], &mut Option::default());

        assert!(actual.is_ok());
        assert_eq!(given, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn compare_equal_eight_position() {
        let mut given = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut output = Option::default();

        let actual = interpret(&mut given, vec![8], &mut output);

        assert!(actual.is_ok());
        assert!(output.is_some());
        assert_eq!(output.unwrap(), 1);
    }

    #[test]
    fn compare_less_than_eight_position() {
        let mut given = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut output = Option::default();

        let actual = interpret(&mut given, vec![7], &mut output);

        assert!(actual.is_ok());
        assert!(output.is_some());
        assert_eq!(output.unwrap(), 1);
    }

    #[test]
    fn compare_equal_eight_immediate() {
        let mut given = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut output = Option::default();

        let actual = interpret(&mut given, vec![8], &mut output);

        assert!(actual.is_ok());
        assert!(output.is_some());
        assert_eq!(output.unwrap(), 1);
    }

    #[test]
    fn compare_less_than_eight_immediate() {
        let mut given = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut output = Option::default();

        let actual = interpret(&mut given, vec![7], &mut output);

        assert!(actual.is_ok());
        assert!(output.is_some());
        assert_eq!(output.unwrap(), 1);
    }
}
