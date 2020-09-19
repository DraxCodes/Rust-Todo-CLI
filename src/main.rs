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

    let command = parse_command_args(arguments);

    match command {
        CommandType::Get => todo_list.print(),
        CommandType::Add(item) => { todo_list.add_to_list(item); todo_list.print(); },
        CommandType::Remove(pos) => { todo_list.remove_item(pos); todo_list.print(); },
        CommandType::Complete(pos) => { todo_list.mark_completed(pos); todo_list.print(); }
        CommandType::Invalid => panic!("You provided invalid input.")
    }
}

fn parse_command_args(args: Vec<String>) -> CommandType {
    return match args[1].to_lowercase().as_str() {
        "get" => CommandType::Get,
        "add" => if args.len() > 2 { 
            CommandType::Add(args[2].clone()) 
        } else { 
            CommandType::Invalid
        },
        "remove" => if args.len() > 2 { 
            CommandType::Remove(args[2].parse::<usize>().unwrap() - 1) 
        } else { 
            CommandType::Invalid
        },
        "completed" => if args.len() > 2 { 
            CommandType::Complete(args[2].parse::<usize>().unwrap() - 1) 
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

    fn remove_item(&mut self, pos: usize) {
        self.items.remove(pos);
    }

    fn mark_completed(&mut self, pos: usize) {
        self.items[pos].completed = 'x';
    }

    fn print(&self) {
        for (pos, item) in self.items.iter().enumerate() {
            println!("{}: [{}] - {}", pos + 1, item.completed, item.name);
        }
    }
}

enum CommandType {
    Get,
    Add(String),
    Remove(usize),
    Complete(usize),
    Invalid
}
