#[derive(Debug, Clone)]
enum Status {
    Pending,
    Done,
}

#[derive(Debug, Clone)]
struct Task {
    description: String,
    status: Status,
}

pub struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String, status: Status) {
        self.tasks.push(Task {
            description,
            status,
        });
    }

    fn remove_task(&mut self, index: usize) -> Option<Task> {
        if index < self.tasks.len() {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }

    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks.");
            return;
        } else {
            for (index, task) in self.tasks.iter().enumerate() {
                println!("{:?}: ({:?}, {:?})", index, task.description, task.status)
            }
        }
    }
}

fn parse_status(s: &str) -> Option<Status> {
    match s.to_lowercase().as_str() {
        "pending" => Some(Status::Pending),
        "done" => Some(Status::Done),
        _ => None,
    }
}

pub fn print_help_doc() {
    println!("Commands:");
    println!("  add <description> <status> - Add a task (status: pending, done)");
    println!("  remove <index> - Remove a task by index");
    println!("  list - List all tasks");
    println!("  quit or q - Exit the program");
    println!("Example: add \"Buy milk and eggs\" pending\n");
}

pub fn handle_add_command(todo: &mut TodoList, args: &[&str]) {
    if args.len() < 3 {
        println!("Usage: add <description> <status>");
        return;
    }

    let description = args[1..args.len() - 1].join(" ");
    let status = parse_status(args.last().unwrap());

    let trimmed_desc = description.trim();
    if trimmed_desc.is_empty() || trimmed_desc == "\"\"" || trimmed_desc == "''" {
        println!("Error: Description cannot be empty or whitespace.");
        return;
    }

    if description.len() > 100 {
        println!("Error: Description too long (max 100 characters).");
        return;
    }

    if let Some(status) = status {
        todo.add_task(description.clone(), status.clone());
        println!("Added: {} ({:?})", description, status);
    } else {
        println!(
            "Invalid status: {}. Use 'pending' or 'done'",
            args.last().unwrap()
        );
    }
}

pub fn handle_remove_command(todo: &mut TodoList, args: &[&str]) {
    if args.len() != 2 {
        println!("Usage: remove <index>");
        return;
    }

    if let Ok(index) = args[1].parse::<usize>() {
        if let Some(removed_task) = todo.remove_task(index) {
            println!("Removed: {:?}", removed_task);
        } else {
            println!("Invalid index: {}", index);
        }
    } else {
        println!("Invalid value {}. Must be a number", args[1]);
    }
}
