use super::errors::OpcodeError::{BadOperator, OutOfBounds};
use super::errors::Result;

pub fn interpret(input: &str) -> Result<String> {
    //parse input
    let mut codes = Vec::new();
    for token in input.split(',') {
        codes.push(token.parse::<u32>()?);
    }
    //executing Opcode
    for i in (0..codes.len()).step_by(4) {
        let operator = codes[i];
        if operator == 99 {
            break;
        }
        if i + 3 >= codes.len() {
            return Err(OutOfBounds(operator as usize));
        }
        let result = lookup_operator(operator, &codes, i)?;
        let destination = usize::try_from(codes[i + 3])?;
        codes[destination] = result;
    }
    //stringify output
    let mut chars = String::from_iter(codes.into_iter().map(|code| code.to_string() + ","));
    chars.pop();
    Ok(chars)
}

fn lookup_operator(operator: u32, codes: &[u32], index: usize) -> Result<u32> {
    let first_pos = usize::try_from(codes[index + 1])?;
    let second_pos = usize::try_from(codes[index + 2])?;
    match operator {
        1 => Ok(codes[first_pos] + codes[second_pos]),
        2 => Ok(codes[first_pos] * codes[second_pos]),
        _ => Err(BadOperator(operator)),
    }
}
