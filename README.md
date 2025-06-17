# TODO CLI

_Rust Learning Journey_

A command line app for creating, reading, updating and deleting tasks.
The tasks are saved into todo_list.json file for persistent.

## Run Locally

Clone the project

```bash
    git clone https://github.com/v-inmar/rust_todo_cli
```

Go to the project directory

```bash
    cd rust_todo_cli
```

Install dependencies

```bash
    cargo build
```

Run the app

```bash
    cargo run Add "this is a task description"
    cargo run List
    cargo run Done "ID123ABC"
    cargo run Del "ID123ABC"
```
