use std::num::{ParseIntError, TryFromIntError};
use std::{error, fmt};

pub type Result<T> = std::result::Result<T, OpcodeError>;

#[derive(Debug)]
pub enum OpcodeError {
    Parse(ParseIntError),
    BadOperator(i32),
    OutOfBounds(usize),
    FailedCast(TryFromIntError),
}

impl fmt::Display for OpcodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpcodeError::Parse(parse_int_error) => write!(f, "{parse_int_error}"),
            OpcodeError::BadOperator(operator) => write!(f, "Operator {operator} is not valid!"),
            OpcodeError::OutOfBounds(index) => write!(f, "Index {index} will reach out of bounds!"),
            OpcodeError::FailedCast(try_from_int_error) => write!(f, "{try_from_int_error}"),
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
            OpcodeError::BadOperator(_) => None,
            OpcodeError::OutOfBounds(_) => None,
            OpcodeError::FailedCast(ref e) => Some(e),
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

impl From<TryFromIntError> for OpcodeError {
    fn from(err: TryFromIntError) -> OpcodeError {
        OpcodeError::FailedCast(err)
    }
}
