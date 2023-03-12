use self::OpcodeError::*;
use std::any::Any;
use std::num::{ParseIntError, TryFromIntError};
use std::{error, fmt};

pub type Result<T> = std::result::Result<T, OpcodeError>;

#[derive(Debug)]
pub enum OpcodeError {
    Parse(ParseIntError),
    BadOperator(i32),
    OutOfBounds(usize),
    FailedCast(TryFromIntError),
    BadParameterMode(i32),
    MissingOutput,
    MissingInput,
    FailedJoin(Box<dyn Any + Send + 'static>),
}

impl fmt::Display for OpcodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Parse(parse_int_error) => write!(f, "{parse_int_error}"),
            BadOperator(operator) => write!(f, "Operator {operator} is not valid!"),
            OutOfBounds(index) => write!(f, "Index {index} will reach out of bounds!"),
            FailedCast(try_from_int_error) => write!(f, "{try_from_int_error}"),
            BadParameterMode(parameter_mode) => {
                write!(f, "Parameter mode {parameter_mode} is not valid!")
            }
            MissingOutput => write!(f, "No output for amplifier!"),
            MissingInput => write!(f, "Input read from too many times!"),
            FailedJoin(any) => write!(f, "Thread join failed for {:?}", any.type_id()),
        }
    }
}

impl error::Error for OpcodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            Parse(ref e) => Some(e),
            BadOperator(_) => None,
            OutOfBounds(_) => None,
            FailedCast(ref e) => Some(e),
            BadParameterMode(_) => None,
            MissingOutput => None,
            MissingInput => None,
            FailedJoin(_) => None,
        }
    }
}

// Implement the conversion from `ParseIntError` to `OpcodeError`.
// This will be automatically called by `?` if a `ParseIntError`
// needs to be converted into a `OpcodeError`.
impl From<ParseIntError> for OpcodeError {
    fn from(err: ParseIntError) -> OpcodeError {
        Parse(err)
    }
}

impl From<TryFromIntError> for OpcodeError {
    fn from(err: TryFromIntError) -> OpcodeError {
        FailedCast(err)
    }
}

impl From<Box<dyn Any + Send>> for OpcodeError {
    fn from(err: Box<dyn Any + Send>) -> OpcodeError {
        FailedJoin(err)
    }
}
