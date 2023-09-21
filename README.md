# oxidatabase

  A Rusty SQLite clone created for learning and fun.

## Project Structure

```bash
  src/
  ├── backend
  │   # Contains backend logic for storage and data management.
  │
  ├── core
  │   ├── metacmds.rs
  │   ├── mod.rs
  │   ├── page.rs
  │   ├── parser.rs
  │   ├── query_engine.rs
  │   ├── record.rs
  │   ├── repl.rs
  │   ├── statement.rs
  │   ├── strings.rs
  │   └── table.rs
  │   # Core components and logic for Oxidatabase.
  │
  └── main.rs
# Main entry point for the Oxidatabase application.
```

## Getting Started

  To get started with oxidatabase, follow the steps:

  1. Clone the repository to your local machine.

  ```bash
    git clone https://github.com/your-username/oxidatabase.git
    cd oxidatabase
  ```

  2. Build and run the application.

  ```bash
    cargo build
    cargo run
  ```

  3. See available commands (in REPL):

  ```bash
    .help
  ```

## License

  This project is licensed under the MIT License - see the `LICENSE` file for details.

## Acknowledgments

  Inspired by [SQLite](https://www.sqlite.org/index.html) and [the series of tutorials/blogs by cstask](https://cstack.github.io/db_tutorial/).

  Andy Pavlo's course [CMU 15-445/645 Fall 2023 Database Systems](https://15445.courses.cs.cmu.edu/fall2023/).

