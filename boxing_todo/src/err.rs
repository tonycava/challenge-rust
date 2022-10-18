pub use std::fmt;
pub use std::fmt::{Debug, Display};
pub use std::error::Error;
use std::fmt::Formatter;
use std::fs;

#[derive(Debug)]
pub enum ParseErr {
    Empty,
    Malformed(Box<dyn Error>),
}

#[derive(Debug)]
pub struct ReadErr {
    pub child_err: Box<dyn Error>,
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseErr::Empty => write!(f, "Fail to parses todo"),
            ParseErr::Malformed(_e) => write!(f, "Fail to parses todo"),
        }
    }
}



impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        let contents = fs::read_to_string("hello.txt")
            .expect("Should have been able to read the file");

        if self.to_string() == "Fail to parses todo" || contents == "empty_tasks.json" {
            return None;
        }
        return Some(self);
    }
}