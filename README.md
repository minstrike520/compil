# awa1

A custom programming language interpreter written in Rust.

## Description

This project is a simple interpreter for a custom programming language. It features a lexer, parser, and an interpreter that evaluates the abstract syntax tree (AST).

## Getting Started

### Prerequisites

- Rust and Cargo must be installed on your system.

### Building

To build the project, run the following command in the root directory:

```bash
cargo build
```

### Running

The interpreter can be run in two modes:

1.  **Shell Mode:**
    To start the interactive shell, run:
    ```bash
    cargo run -- shell
    ```
    You can then enter expressions at the prompt. To exit the shell, type `exit`.

2.  **File Mode:**
    To execute a script file, run:
    ```bash
    cargo run -- <file_path>
    ```
    Replace `<file_path>` with the path to your script file.

## Usage

The language supports basic expressions. Here is an example of what you can do in the shell:

```
> 1 + 2
NumberValue(3)
```
