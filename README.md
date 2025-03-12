# Todo CLI - A Simple To-Do List Manager in Rust

**Todo CLI** is a command-line application written in Rust for managing your to-do list. With this CLI, you can add, list, mark as completed, remove, export, and import tasks from your to-do list, all from the comfort of your terminal.

---

## Features

- **Add tasks**: Add new tasks to your to-do list.
- **List tasks**: View all pending tasks.
- **Complete tasks**: Mark a task as completed.
- **Remove tasks**: Delete a task from the list.
- **Export tasks**: Export all your tasks to a JSON file.
- **Import tasks**: Import tasks from a JSON file.
- **Dry Run mode**: Allows importing tasks without making any changes to your list.

---

## Requirements

- [Rust](https://www.rust-lang.org/) 1.50 or higher.
- Cargo (Rust's package manager, included with Rust).

---

## Installation

To install and use the application, follow these steps:

1. **Clone the repository**:

   ```bash
   git clone https://github.com/DelgadoElias/ede-todo.git
   cd ede-todo
   ```

2. **Build the project**:

   ```bash
   cargo build --release
   ```

3. **Install the binary system-wide** (optional):

   ```bash
   cargo install --path .
   ```

   This will install the `ede-todo` binary globally on your system.

---

## Usage

Once installed, you can start using the CLI with the following commands:

### 1. Add a Task

To add a new task to your to-do list:

```bash
ede-cli add --task "My new task"
```

### 2. List Tasks

To view all pending tasks:

```bash
ede-cli list
```

### 3. Complete a Task

To mark a task as completed (for example, task ID 1):

```bash
ede-cli complete --id 1
```

### 4. Remove a Task

To delete a task from the list (for example, task ID 1):

```bash
ede-cli remove --id 1
```

### 5. Export Tasks to a JSON File

To export all tasks to a file named `tasks.json` (you can change it passing --file arg, but for default is tasks.json):

```bash
ede-cli export --file tasks.json
```

### 6. Import Tasks from a JSON File

To import tasks from a JSON file (`tasks.json`):

```bash
ede-cli import --file tasks.json
```

#### Dry Run Mode

If you only want to see which tasks would be imported without making changes, you can use the dry run mode:

```bash
ede-cli import --file tasks.json --dry-run
```

---

## Example Usage

### Add and List Tasks

```bash
$ ede-cli add --task "Buy milk"
✅ Task added: Buy milk

$ ede-cli list
📭 No pending tasks.

$ ede-cli add --task "Call the doctor"
✅ Task added: Call the doctor

$ ede-cli list
📋 1 - Call the doctor
```

### Complete and Remove Tasks

```bash
$ ede-cli complete --id 1
✅ Task 1 marked as completed

$ ede-cli remove --id 1
🗑️ Task 1 deleted
```

---

## Contributing

Contributions are welcome! If you'd like to help improve this project, you can:

1. Fork the repository.
2. Create a branch for your change: `git checkout -b feature/new-feature`.
3. Make your changes.
4. Make sure all tests pass: `cargo test`.
5. Commit your changes: `git commit -m 'Add new feature'`.
6. Push your branch: `git push origin feature/new-feature`.
7. Open a pull request.

---

## Testing

To run the project's tests, use the following command:

```bash
cargo test
```

This will run all the tests and display the results in your terminal.

I'm trying to use tarpaulin with almost 80% of code coverage but I think I'm not doing well :c

---

## License

This project is licensed under the MIT License. For more details, please refer to the [LICENSE](LICENSE) file.

---

## Development

If you want to contribute to the development of this project, note that the source code is written in Rust and uses the **Cargo** package manager. Here are some additional details for setting up your development environment:

1. **Install Rust**: If you haven't installed Rust yet, visit [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2. **Recommended Editor**: Use any text editor that supports Rust. I personally like Nvim with AstroNvim set.
3. **Documentation**: For more details on using Rust, refer to the [official Rust documentation](https://doc.rust-lang.org/book/).

---

Thanks for using Todo CLI! If you have any suggestions or find a bug, please open an issue on GitHub.
