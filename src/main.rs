use crate::OpcodeError::{BadOperator, OutOfBounds};
use std::num::ParseIntError;
use std::{error, fmt};

fn main() {
    let input = String::from("1,9,10,3,2,3,11,0,99,30,40,50");
    let result = interpret(&input);
    match result {
        Ok(output) => println!("{output}"),
        Err(error) => eprintln!("{error}"),
    }
}

type Result<T> = std::result::Result<T, OpcodeError>;

#[derive(Debug)]
enum OpcodeError {
    Parse(ParseIntError),
    BadOperator(u32),
    OutOfBounds(usize),
}

impl fmt::Display for OpcodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpcodeError::Parse(parse_int_error) => write!(f, "{parse_int_error}"),
            BadOperator(operator) => write!(f, "Operator {operator} is not valid!"),
            OutOfBounds(index) => write!(f, "Index {index} will reach out of bounds!"),
        }
    }
}

impl error::Error for OpcodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            OpcodeError::Parse(ref e) => Some(e),
            BadOperator(_) => None,
            OutOfBounds(_) => None,
        }
    }
}

// Implement the conversion from `ParseIntError` to `OpcodeError`.
// This will be automatically called by `?` if a `ParseIntError`
// needs to be converted into a `OpcodeError`.
impl From<ParseIntError> for OpcodeError {
    fn from(err: ParseIntError) -> OpcodeError {
        OpcodeError::Parse(err)
    }
}

fn interpret(input: &str) -> Result<String> {
    //parse input
    let mut codes: Vec<u32> = Vec::new();
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
        let first_pos = codes[i + 1] as usize;
        let second_pos = codes[i + 2] as usize;
        let result = match operator {
            1 => codes[first_pos] + codes[second_pos],
            2 => codes[first_pos] * codes[second_pos],
            _ => return Err(BadOperator(operator)),
        };
        let destination = codes[i + 3];
        codes[destination as usize] = result;
    }
    //stringify output
    let mut chars = codes
        .into_iter()
        .map(|code| code.to_string() + ",")
        .collect::<String>();
    chars.pop();
    Ok(chars)
}
