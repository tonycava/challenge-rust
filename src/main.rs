pub use boxing_todo::TodoList;

fn main() {
    let todos = TodoList::get_todo("boxing_todo/src/malformed_objec.json");
    match todos {
        Ok(list) => println!("{:?}", list),
        Err(e) => {
            println!("{}{:?}", e.to_string(), e.source());
        }
    }
    let todos = TodoList::get_todo("boxing_todo/src/malformed_objec.json");
    match todos {
        Ok(list) => println!("{:?}", list),
        Err(e) => {
            println!("{}{:?}", e.to_string(), e.source());
        }
    }
}