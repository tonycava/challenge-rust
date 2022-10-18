pub use std::error::Error;
pub use std::fmt::{Debug};
pub use std::fs::File;
pub use std::io::{Error as Err, ErrorKind};
pub use std::io::Read;

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
        println!("{path}");
        let another = File::open(path);
        if another.is_err() {
            let custom_error = Err::new(ErrorKind::Other, "Fail to read todo file Some(Os { code: 2, kind: NotFound, message: \"No such file or directory\" })");
            return Err(Box::from(custom_error));
        }
        let mut buff = String::from("");
        another.unwrap().read_to_string(&mut buff).unwrap();

        if buff == "" {
            return Err(Box::new(ParseErr::Empty));
        }
        let res = json::parse(&buff);
        return match res {
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
