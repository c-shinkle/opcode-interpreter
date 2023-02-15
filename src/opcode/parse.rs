use crate::opcode::errors::{OpcodeError, Result};

pub fn imperative(codes_string: &str) -> Result<Vec<i32>> {
    let mut codes = Vec::new();
    for token in codes_string.split(',') {
        codes.push(token.parse::<i32>()?);
    }
    Ok(codes)
}

pub fn functional_parse(codes_string: &str) -> Result<Vec<i32>> {
    codes_string
        .split(',')
        .map(|token| token.parse::<i32>().map_err(OpcodeError::from))
        .collect()
}
