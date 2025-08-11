# Rust CLI Todo App 🦀

A simple command-line Todo application written in Rust with persistent JSON storage.

---

## Features

- Add new tasks with a name (default status: not completed)
- Update existing tasks’ name and completion status
- Remove tasks by their number
- Show all tasks with clear ✅/❌ status icons
- Save tasks persistently to a JSON file (`tasks.json`)
- Load tasks from the JSON file on startup
- Clear all tasks at once
- Robust input validation and user-friendly prompts

---

## How to Run

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed (cargo & rustc)

### Steps

1. Clone this repository or copy the source code to your local machine.

2. Navigate to the project folder in your terminal.

3. To **build and run the project** directly:

```bash
cargo run
```