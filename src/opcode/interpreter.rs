use super::errors::OpcodeError::*;
use super::errors::Result;
use num_traits::FromPrimitive;

#[derive(FromPrimitive, PartialEq)]
pub enum Operator {
    Addition = 1,
    Multiplication = 2,
    Terminate = 99,
}

pub fn interpret(input: &str) -> Result<String> {
    //parse input
    let mut codes = Vec::new();
    for token in input.split(',') {
        codes.push(token.parse::<u32>()?);
    }
    //executing Opcode
    for i in (0..codes.len()).step_by(4) {
        let operator = FromPrimitive::from_u32(codes[i]).ok_or(BadOperator(codes[i]))?;

        if operator == Operator::Terminate {
            break;
        }
        if i + 3 >= codes.len() {
            return Err(OutOfBounds(i));
        }

        let destination = usize::try_from(codes[i + 3])?;
        codes[destination] = lookup_operator(operator, &codes, i)?;
    }
    //stringify output
    let mut chars = codes
        .into_iter()
        .map(|code| code.to_string() + ",")
        .collect::<String>();
    chars.pop();
    Ok(chars)
}

fn lookup_operator(operator: Operator, codes: &[u32], index: usize) -> Result<u32> {
    let first_pos = usize::try_from(codes[index + 1])?;
    let second_pos = usize::try_from(codes[index + 2])?;
    match operator {
        Operator::Addition => Ok(codes[first_pos] + codes[second_pos]),
        Operator::Multiplication => Ok(codes[first_pos] * codes[second_pos]),
        Operator::Terminate => unreachable!("The 'Terminate' operator was checked upstream!"),
    }
}
