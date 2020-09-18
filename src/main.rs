use std::env;

struct TodoItem {
    name: String,
    completed: char
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem {
            name: name,
            completed: ' '
        };
    }
}

struct TodoList {
    items: Vec<TodoItem>
}

impl TodoList {
    fn new() -> TodoList {
        return TodoList { items: Vec::new() };
    }

    fn add_to_list(&mut self, name: String) {
        self.items.push(TodoItem::new(name));
    }

    fn print(&self) {
        for item in &self.items {
            println!("[{}] - {}", item.completed, item.name);
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let mut todo_list = TodoList::new();
    todo_list.add_to_list("Buy Milk".to_string());
    todo_list.add_to_list("Buy Bread".to_string());

    

    if arguments.len() < 2 {
        println!("You need to provide a comamnd.");
        return;
    }

    let command = arguments[1].clone().to_lowercase();

    if command == "get" {
        todo_list.print();
    } else if command == "add" && arguments.len() > 2{
        let task = arguments[2].clone();
        todo_list.add_to_list(task.to_string());
        todo_list.print();
    }
}
