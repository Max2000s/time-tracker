# time-tracker
The Time Tracker CLI is a command-line tool to help you track the time spent on various tasks. 
You can start, stop, and list tasks, and see detailed information about each task, including the current duration of active tasks and the history of completed tasks.

## Features

- Start a new task or continue an existing task.
- Stop the current active task.
- List all tasks with detailed information in a tabular format.
- Store task data in a JSON file.

## Installation

### Prerequisites

- Rust programming language: [Install Rust](https://www.rust-lang.org/tools/install)

### Build

1. Clone the repository:
    ```bash
    git clone https://github.com/Max2000s/time-tracker.git
    cd time-tracker
    ```

2. Build the project:
    ```bash
    cargo build --release
    ```

3. The binary will be located in the `target/release` directory.

### Install Using ZIP Folder

 - Get the `time_tracker.zip` file.
 - Extract the ZIP file:
   ```bash
   unzip task_time_tracker.zip -d task_time_tracker
   ```

 - Move the binary to a directory in the PATH, such as `/usr/local/bin`:
   ```bash
   sudo mv task_time_tracker/task_time_tracker /usr/local/bin/
   sudo chmod +x /usr/local/bin/task_time_tracker
   ```

- Run the tool:
```bash
task_time_tracker --help
```


## Commands

### Start a Task

Start a new task or continue an existing task.

```bash
time-tracker start <task_name>
```

### Stop an active Task

Stop an active task (since only one task can be active at one time).

```bash
time-tracker start
```

### List all Tasks

List all the tasks that were created and gather information for each of them.

```bash
time-tracker list
```

### Delete a Task

Deletes an existing task. If it is currently activated it will also be deleted and as a consequence there is no active task anymore.

```bash
time-tracker delete <task_name>
```

## Development

### Project Structure

- src/main.rs: The main entry point of the application.
- src/commands/: Contains the command implementations (start.rs, stop.rs, list.rs).
- src/models/: Contains the data models (task.rs).
- src/persistence/: Contains the persistence logic (file_storage.rs).

### Dependencies

- clap: For command-line argument parsing.
- chrono: For date and time handling.
- serde: For serialization and deserialization of data.
- serde_json: For working with JSON data.
- dirs: For cross-platform directory handling.
- prettytable-rs: For displaying tasks in a tabular format.
