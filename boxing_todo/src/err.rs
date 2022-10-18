pub use std::fmt;
pub use std::fmt::{Debug, Display};
pub use std::error::Error;

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

impl Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.child_err.to_string() != "" {
            return write!(f, "Fail to parse todo");
        }
        write!(f, "Fail to parses todo file")
    }
}

impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if Some(self).unwrap().to_string() != "Fail to parses todo" {
            return None;
        }
        return Some(self);
    }
}

impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        println!("la");
        return Some(self);
    }
}