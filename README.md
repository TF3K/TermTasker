
# Mortal Reminder V1.0b

A light-weight todo list CLI written in rust, allowing for users to add, list, mark and remove tasks with ease, using Serde for JSON file storage for persistant data.
## Features

- Serialization and Deserialization: Utilizes serde and serde_json for efficient data serialization and deserialization of task data.
- Command-line Styling: Enhances user experience with colorful and styled CLI interface using owo_colors.
- Date and Time Handling: Incorporates chrono for precise handling of dates and times in task management.
- UUID Generation: Employs uuid to generate universally unique identifiers (UUIDs) for tasks.


#### Main Components:
- Task Structs and Enums: Defines structures and enums for tasks, statuses, and priorities.
- User Interaction Functions: Implements functions for user input, console screen clearing, and user prompts.
- File Management: Manages tasks stored in a JSON file (db/tasks.json), enabling read, write, and update operations.
- Main Function: Orchestrates the CLI functionality, offering a menu for users to add, list, mark, or remove tasks.
- Conditional Logic: Handles file paths based on the operating system using platform-specific conditional compilation.
- Error Handling: Ensures robustness with error handling for file operations, JSON parsing, and user input.




## Installation
After cloning the repo run the following commands:

On linux:
```bash
  cargo build --release
```

On windows:
```cmd
    cargo build --release
```

#### Creating an alias for the executable:

On linux:
```bash
    alias mr="/target/release/mortal_reminder"
    or
    alias mortalreminder="/target/release/mortal_reminder"
```

On windows:
```cmd
    doskey mr=target\release\mortal_reminder.exe
```


    
# Future Goals

- Add support for MacOS
- Expand more options in the CLI
- Add searching for tasks by ID/UUID
- Optimize serialization / deserialization process

