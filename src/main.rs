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

    let command = parse_command_args(&arguments);

    match command {
        CommandType::Get => todo_list.print(),
        CommandType::Add => { todo_list.add_to_list(arguments[2].clone()); todo_list.print(); },
        CommandType::Remove => { todo_list.remove_item(arguments[2].parse::<usize>().unwrap() - 1); todo_list.print(); },
        CommandType::Complete => { todo_list.mark_completed(arguments[2].parse::<usize>().unwrap() - 1); todo_list.print(); }
        CommandType::Invalid => panic!("You provided invalid input.")
    }
}

fn parse_command_args(args: &Vec<String>) -> CommandType {
    return match args[1].to_lowercase().as_str() {
        "get" => verify_args_and_set_command_type(1, CommandType::Get, &args),
        "add" => verify_args_and_set_command_type(2, CommandType::Add, &args),
        "remove" => verify_args_and_set_command_type(2, CommandType::Remove, &args),
        "complete" => verify_args_and_set_command_type(2, CommandType::Complete, &args),
        _ => panic!("You must provide a valid command.")
    };
}

fn verify_args_and_set_command_type(length: usize, command_type: CommandType, args: &Vec<String>) -> CommandType {
    if args.len() > length {
        return command_type;
    } else {
        return CommandType::Invalid;
    }
}