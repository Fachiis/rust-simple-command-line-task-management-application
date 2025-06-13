#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_list_new() {
        let todo = TodoList::new();
        assert_eq!(todo.tasks.len(), 0, "New TodoList should be empty")
    }

    #[test]
    fn test_todo_add_task() {
        let mut todo = TodoList::new();
        todo.add_task(String::from("Buy milk"), Status::Pending);
        assert_eq!(todo.tasks.len(), 1, "Should have one task");
        assert_eq!(
            todo.tasks[0].description, "Buy milk",
            "Description should match"
        );
        assert_eq!(
            todo.tasks[0].status,
            Status::Pending,
            "Status should be pending"
        )
    }

    #[test]
    fn test_remove_task_valid() {
        let mut todo = TodoList::new();
        todo.add_task(String::from("Buy fish"), Status::Done);
        todo.remove_task(0);
        assert_eq!(todo.tasks.len(), 0, "TodoList should be empty")
    }

    #[test]
    fn test_remove_task_invalid() {
        let mut todo = TodoList::new();
        todo.add_task(String::from("Buy fish"), Status::Done);
        let removed = todo.remove_task(1);
        assert!(removed.is_none(), "Should return None for invalid index"); // Assert here
        assert_eq!(todo.tasks.len(), 1, "TodoList should still have one task")
    }

    #[test]
    fn test_handle_add_command_valid() {
        let mut todo = TodoList::new();
        let args = vec!["add", "buy milk", "pending"];
        handle_add_command(&mut todo, &args);
        assert_eq!(todo.tasks.len(), 1, "Should add one task");
        assert!(
            matches!(todo.tasks[0].status, Status::Pending),
            "Status should be pending"
        ); // Match pattern
    }

    #[test]
    fn test_add_command_empty_description() {
        let mut todo = TodoList::new();
        let args = vec!["add", "", "done"];
        handle_add_command(&mut todo, &args);
        assert_eq!(
            todo.tasks.len(),
            0,
            "Should not add task with empty description"
        );
    }

    #[test]
    fn test_add_command_too_long_description() {
        let mut todo = TodoList::new();
        let long_desc = "a".repeat(101);
        let args = vec!["add", &long_desc, "pending"];
        handle_add_command(&mut todo, &args);
        assert_eq!(
            todo.tasks.len(),
            0,
            "Should not add task with too long description"
        );
    }

    #[test]
    fn test_handle_add_command_invalid_status() {
        let mut todo = TodoList::new();
        let args = vec!["add", "Buy milk", "invalid"];
        handle_add_command(&mut todo, &args);
        assert_eq!(
            todo.tasks.len(),
            0,
            "Should not add task with invalid status"
        );
    }

    #[test]
    fn test_handle_remove_command_valid() {
        let mut todo = TodoList::new();
        todo.add_task(String::from("Buy milk"), Status::Pending);
        let args = vec!["remove", "0"];
        handle_remove_command(&mut todo, &args);
        assert_eq!(todo.tasks.len(), 0, "Should remove task");
    }

    #[test]
    fn test_handle_remove_command_invalid_index() {
        let mut todo = TodoList::new();
        todo.add_task(String::from("Buy milk"), Status::Pending);
        let args = vec!["remove", "1"];
        handle_remove_command(&mut todo, &args);
        assert_eq!(
            todo.tasks.len(),
            1,
            "Should not remove task for invalid index"
        );
    }

    #[test]
    fn test_parse_status() {
        assert!(matches!(parse_status("pending"), Some(Status::Pending)));
        assert!(matches!(parse_status("Pending"), Some(Status::Pending)));
        assert!(matches!(parse_status("done"), Some(Status::Done)));
        assert!(matches!(parse_status("Done"), Some(Status::Done)));
        assert!(parse_status("invalid").is_none());
    }
}
