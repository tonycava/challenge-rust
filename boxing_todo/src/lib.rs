use std::error::Error;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

mod err;

use err::{ParseErr, ReadErr};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let mut another = File::open(path).unwrap();
        let mut buff = String::from("");
        another.read_to_string(&mut buff).unwrap();

        if buff == "" {
            return Err(Box::new(ParseErr::Empty));
        }

        let res = serde_json::from_str::<TodoList>(&buff);

        return match res {
            Ok(v) => Ok(TodoList {
                title: v.title,
                tasks: v.tasks,
            }),
            Err(e) => Err(Box::new(ParseErr::Malformed(Box::new(ReadErr {
                child_err: Box::new(e)
            }))))
        };
    }
}
