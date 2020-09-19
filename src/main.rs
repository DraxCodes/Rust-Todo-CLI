use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let mut todo_list = TodoList::new();
    todo_list.add_to_list("Buy Milk".to_string());
    todo_list.add_to_list("Buy Bread".to_string());

    

    if arguments.len() < 2 {
        println!("You need to provide a comamnd.");
        return;
    }

    let command = parse_command_arg(arguments);

    match command {
        CommandType::Get => todo_list.print(),
        CommandType::Add(item) => todo_list.add_to_list(item),
        CommandType::Invalid => panic!("You provided invalid input.")
    }
}

fn parse_command_arg(args: Vec<String>) -> CommandType {
    return match args[1].to_lowercase().as_str() {
        "get" => CommandType::Get,
        "add" => if args.len() > 2 { 
            CommandType::Add(args[2].clone()) 
        } else { 
            CommandType::Invalid
        },
        _ => panic!("You must provide a valid command.")
    };
}

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

enum CommandType {
    Get,
    Add(String),
    Invalid
}
