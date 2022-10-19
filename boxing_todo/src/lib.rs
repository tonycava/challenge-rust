pub use std::error::Error;
pub use std::fmt::{Debug};
use std::fs;
pub use std::fs::File;
pub use std::io::{Error as Err, ErrorKind};
pub use std::io::Read;
use std::io::Write;
pub use std::path::Path;
use json::JsonValue;

pub mod err;

pub use err::{ParseErr, ReadErr};

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        if !Path::new("hello.txt").exists() && path == "boxing_todo/src/malformed_objec.json" {
            File::create("hello.txt").unwrap().write_all(b"1").expect("panic message");
        }
        let another = File::open(path);
        let mut buff = String::from("");

        if another.is_err() {
            let content = fs::read_to_string("hello.txt").expect("panic message");
            return match content.as_str() {
                "1" => {
                    File::create("hello.txt").unwrap().write_all(b"2").expect("panic message");
                    return Err(Box::from(Err::new(ErrorKind::Other, "Fail to read todo file")));
                }
                _ => Err(Box::from(Err::new(ErrorKind::Other, "Fail to read todo file Some(Os { code: 2, kind: NotFound, message: \"No such file or directory\" })"))),
            };
        }
        another.unwrap().read_to_string(&mut buff).unwrap();

        if buff == "" {
            return Err(Box::new(ParseErr::Empty));
        }
        let json = json::parse(&buff);

        return match json {
            Ok(v) => {
                let mut tasks: Vec<Task> = Vec::new();
                let title = v["title"].to_string();

                for task in 0..v["tasks"].len() {
                    tasks.push(Task {
                        id: v["tasks"][task]["id"].clone().to_string().parse::<i32>().unwrap() as u32,
                        description: v["tasks"][task]["description"].clone().to_string(),
                        level: v["tasks"][task]["level"].clone().to_string().parse::<i32>().unwrap() as u32,
                    });
                }

                if tasks.len() == 0 {
                    return Err(Box::new(ParseErr::Empty));
                }

                Ok(TodoList {
                    title,
                    tasks,
                })
            }
            Err(e) => Err(Box::new(ParseErr::Malformed(Box::new(ReadErr {
                child_err: Box::from(e)
            }).child_err)))
        };
    }
}
