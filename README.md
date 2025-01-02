# CLI Rust To-Do List

A simple command-line interface (CLI) to-do list application, built in Rust, designed to be integrated into the larger **Rust Personal CLI Manager** project. This application provides a lightweight way to manage tasks and stay organized directly from your terminal.

## Features

- **Add tasks**: Easily add new tasks to your to-do list.
- **List tasks**: View all tasks in your list with details.
- **Mark tasks as done**: Quickly mark tasks as completed.
- **Delete tasks**: Remove tasks from the list when they are no longer needed.
- **Persistence**: Task data is saved and persists across application restarts.

## Technology Stack

- **Language**: Rust
- **Data Storage**: Local file-based storage (JSON or simple text file)

## Getting Started

### Prerequisites

To run this project, you need:

- **Rust**: Ensure that you have the latest version of Rust installed. You can install Rust using [rustup](https://rustup.rs/).

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/cli-rust-to-do-list.git
   cd cli-rust-to-do-list
   cargo run
