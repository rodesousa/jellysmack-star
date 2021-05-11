use std::fmt;
use std::process;

// errors
pub enum StarError {
    InputError,
    EqualSizeError,
    FormatError,
    CannotSwitchError,
    NoSolutionError,
    LengthError(usize),
}

impl fmt::Display for StarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StarError::InputError => write!(f, "An error occured during reading"),
            StarError::EqualSizeError => write!(f, "The two entries aren't the same size"),
            StarError::FormatError => write!(f, "One of both inputs doesn't comply"),
            StarError::NoSolutionError => write!(f, "No solution was found"),
            StarError::CannotSwitchError => write!(f, "No rules can be applied"),
            StarError::LengthError(count) => {
                write!(f, "{}, Length not respected (1 >= lenght >= 25)", count)
            }
        }
    }
}

// uniq function to exit
pub fn exit(error: StarError) {
    eprintln!("{}", error);
    process::exit(1);
}
