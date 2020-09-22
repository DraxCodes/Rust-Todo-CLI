mod todo;

use std::env;
use todo::CommandType;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let mut todo_list = todo::TodoList::new();
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

