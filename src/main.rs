use std::io;
use std::io::Write;
use todo_cli::{handle_add_command, handle_remove_command, print_help_doc, TodoList};

fn main() {
    println!("\nWelcome to the TODO CLI project\n");
    print_help_doc();

    let mut todo = TodoList::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed

        let mut user_input = String::new();
        let lines = io::stdin().read_line(&mut user_input);

        match lines {
            Ok(_) => {
                let args: Vec<&str> = user_input.trim().split_whitespace().collect();

                if args.is_empty() {
                    continue;
                }

                match args[0].to_lowercase().as_str() {
                    "quite" | "q" => {
                        println!("Goodbye!");
                        break;
                    }
                    "help" => print_help_doc(),
                    "add" => handle_add_command(&mut todo, &args),
                    "remove" | "rm" => handle_remove_command(&mut todo, &args),
                    "list" | "ls" => todo.list_tasks(),
                    _ => {
                        println!("Unknown command: {}. Type 'help' for commands.", args[0]);
                    }
                }
            }
            Err(e) => {
                println!("Error reading input: {}", e);
                continue;
            }
        }
    }
}
