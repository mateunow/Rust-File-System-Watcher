# Rust-project
Authors: 
Maja Sankowska
Mateusz Nowak


# File System Watcher (Rust)

Lightweight CLI tool for monitoring file system changes in real time using an event-driven architecture.  
Built with Rust to explore low-level systems programming, concurrency, and efficient event handling.

---

## Overview

This project is a file system observer that watches a given directory (recursively) and logs events such as:

- file creation
- file modification
- file deletion
- file access (read/write/execute)
- directory changes

The application is designed to demonstrate:
- event-driven programming
- system-level file monitoring
- safe concurrency in Rust
- handling asynchronous OS events

---

## Tech Stack

- **Language:** Rust
- **Library:** `notify` (cross-platform filesystem notifications)
- **Paradigm:** Event-driven / observer pattern

---

## Features

- Recursive directory monitoring
- Real-time event handling
- Differentiation between files and directories
- Detection of:
  - Create / Modify / Remove events
  - Access events (read/write/execute)
- Clear CLI logging output
- Safe error handling using `Result<T, E>`

---

## How It Works

The application uses the `notify` crate to register a watcher on a given path.

1. The program initializes a watcher using `recommended_watcher`
2. It subscribes to file system events in **recursive mode**
3. Each event is passed to a handler function
4. Events are parsed and categorized based on:
   - type (create, modify, remove, access)
   - target (file or directory)
5. Relevant information is printed to the console

---

## Usage

### 1. Clone the repository

```bash
git clone https://github.com/mateunow/Rust-project.git
cd Rust-project
```

2. Run the project
cargo run -- <path_to_watch>

Example:
```bash
cargo run -- ./test_directory
```
Example Output
```bash
Starting watcher on path: "./test_directory"
File created in path: "./test_directory/file.txt"
File modified in path: "./test_directory/file.txt"
File removed in path: "./test_directory/file.txt"
```
## Project Structure
src/

 └── main.rs      # Entry point, watcher initialization, event handling
 
## Key Concepts Demonstrated

- Rust ownership and borrowing in async/event context
- Pattern matching (`match`) for event handling
- Error handling with `Result`
- Working with OS-level events
- Separation of concerns (watcher vs handler)

## Possible Improvements

- Structured logging (e.g. JSON output)
- Filtering by file type or extension
- Integration with a message queue (Kafka / RabbitMQ)
- File change persistence (database logging)
- CLI argument parsing (e.g. `clap`)
- Multi-threaded event processing

## Motivation

The goal of this project was to better understand:

- how operating systems expose file system events
- how Rust handles real-time event streams safely
- how to build efficient, low-level tools with minimal overhead
