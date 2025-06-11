# TODO CLI

A simple command-line task management application written in Rust.

## Overview

TODO CLI is a lightweight, terminal-based task management tool that allows you to keep track of your tasks with simple
commands. The application provides basic functionality to add, remove, and list tasks along with their statuses.

## Features

- Add new tasks with descriptions and statuses (pending or done)
- Remove tasks by index
- List all tasks with their statuses
- Simple and intuitive command-line interface

## Installation

### Prerequisites

- Rust and Cargo installed on your system ([install Rust](https://www.rust-lang.org/tools/install))

### Building from Source

1. Clone this repository:
   ```
   git clone [repository-url]
   cd todo_cli
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the application:
   ```
   cargo run
   ```

## Usage

Once you start the application, you'll be presented with a prompt (`>`). Here are the available commands:

### Commands

- `add <description> <status>` - Add a new task
    - `description`: Text describing the task (max 100 characters)
    - `status`: Either `pending` or `done`
    - Example: `add Buy milk and eggs pending`

- `remove <index>` or `rm <index>` - Remove a task by its index
    - `index`: The numeric index of the task (shown when listing tasks)
    - Example: `remove 0` or `rm 0`

- `list` or `ls` - List all tasks with their indices and statuses

- `help` - Display help information
- `quit` or `q` - Exit the application

### Example Session

```
> add Buy groceries pending
Added: Buy groceries (Pending)
> add Complete project report pending
Added: Complete project report (Pending)

> list
0: ("Buy groceries", Pending)
1: ("Complete project report", Pending)

> remove 0
Removed: Task { description: "Buy groceries", status: Pending }

> list
0: ("Complete project report", Pending)

> quit
Goodbye!
```

## Testing

This project includes comprehensive unit tests for the core functionality. The tests ensure that:

- A new todo list starts empty
- Tasks are added with correct descriptions and statuses
- Tasks can be removed by valid index
- Removing a task with an invalid index does not modify the list and returns `None`

To run the tests, use:

```
cargo test
```

Example test output:

```
running 4 tests
test tests::test_todo_list_new ... ok
test tests::test_todo_add_task ... ok
test tests::test_remove_task_valid ... ok
test tests::test_remove_task_invalid ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Project Structure

- `src/main.rs` - Application entry point and command handling
- `src/lib.rs` - Core functionality implementation (TodoList, Task, and Status)

## Future Enhancements

- [ ] Save tasks to a file for persistence between sessions
- [ ] Add due dates for tasks
- [ ] Mark tasks as done without removing them
- [ ] Add priority levels for tasks
- [ ] Add categories/tags for tasks
- [ ] Search functionality

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
