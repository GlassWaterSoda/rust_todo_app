#[derive(Debug)]
struct Todo{
    id: i32,
    title: String,
    completed: bool,
}

struct Todolist{
    todos: Vec<Todo>,
    next_id: i32
}

impl Todolist{
    fn new() -> Self{
        Todolist { todos: Vec::new(), next_id: 1 }
    }

}


fn main() {
    println!("Hello, world!");

    let todo1 = Todo{
        id: 1,
        title: "Build a TODO APP".to_string(),
        completed: true
    };

    println!("{:?}", todo1)
}
