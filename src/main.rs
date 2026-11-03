use std::fmt;

#[derive(Debug, Clone)]
struct Todo{
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Debug)]
struct Todolist{
    todos: Vec<Todo>,
    next_id: i32
}

impl Todolist{
    fn new()-> Self{
        Todolist { todos: Vec::new(), next_id: 1 }
    }

    fn insert_todo(&mut self, title: &str)-> i32{

        let id = self.next_id;
        let todo = Todo{

            id,
            title: title.to_string(),
            completed: false
        };

        self.todos.push(todo);
        self.next_id += 1;

        id

    } 

    fn get_all_todo(&mut self) -> Vec<&Todo>{
        self.todos.iter().collect()
       
    }

    fn get_mut_todo(&mut self, id:i32)-> Option<&mut Todo>{

        self.todos.iter_mut().find(|t| t.id == id)
    }

    fn update_todo(&mut self, id:i32) -> Option<&Todo>{

        let todo = self.get_mut_todo(id);

        if let Some(x) = todo  {
            x.completed = !x.completed;
        }

        self.todos.iter().find(|t|t.id == id)

    }

    fn delete_todo(&mut self, id:i32) -> i32 {

        self.todos.retain(|c| c.id != id);

        id
    }



}

impl fmt::Display for Todolist{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for t in &self.todos {
            writeln!(f, "id: {} | title: {} | completed: {}", t.id, t.title, t.completed)?;
        }
        Ok(())
    }
}

impl fmt::Display for Todo{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "id: {} | title: {} | completed: {}", self.id, self.title, self.completed)
    }
}


fn main() {
    println!("Hello, world!");

    let mut  list = Todolist::new();


    let todo1 = Todo{
        id: 1,
        title: "Build a TODO APP".to_string(),
        completed: true
    };

    list.insert_todo("Building a Todo APP");
    list.insert_todo("Building a Blog Post");
    list.insert_todo("Building a Tik Tac Toe");
    list.insert_todo("Building a Mike JOhson");


    println!("{}", list);

    list.get_all_todo().iter().for_each(|c|
        println!("id : {} | title: {} | completed: {}", c.id, c.title, c.completed)
    );

    println!("{:?}", list.get_mut_todo(2));

    println!("{:?}", list.delete_todo(2));

     list.get_all_todo().iter().for_each(|c|
        println!("id : {} | title: {} | completed: {}", c.id, c.title, c.completed)
    );


}
