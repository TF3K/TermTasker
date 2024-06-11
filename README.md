
# TermTask (formerly known as Mortal Reminder) V1.0b

A lightweight to-do list CLI written in Rust allows users to easily add, list, mark and remove tasks, using Serde for JSON file storage for persistent data.
## Features

- Serialization and Deserialization: Utilizes serde and serde_json for efficient data serialization and deserialization of task data.
- Command-line Styling: Enhances user experience with a colorful and styled CLI interface using owo_colors.
- Date and Time Handling: Incorporates chrono for precisely handling dates and times in task management.
- UUID Generation: Employs UUID to generate universally unique identifiers (UUIDs) for tasks.


#### Main Components:
- Task Structs and Enums: Defines structures and enums for tasks, statuses, and priorities.
- User Interaction Functions: Implements functions for user input, console screen clearing, and user prompts.
- File Management: Manages tasks in a JSON file (db/tasks.json), enabling read, write, and update operations.
- Main Function: Orchestrates the CLI functionality, offering a menu for users to add, list, mark, or remove tasks.
- Conditional Logic: Handles file paths based on the operating system using platform-specific conditional compilation.
- Error Handling: Ensures robustness with error handling for file operations, JSON parsing, and user input.




## Installation
After cloning the repo run the following commands:

On Linux:
```bash
  cargo build --release
```

On Windows:
```cmd
    cargo build --release
```

#### Creating an alias for the executable:

On Linux:
```bash
    alias mr="/target/release/mortal_reminder"
    or
    alias mortalreminder="/target/release/mortal_reminder"
```

On Windows:
```cmd
    doskey mr=target\release\mortal_reminder.exe
```


    
# Future Goals

- Add support for MacOS
- Expand more options in the CLI
- Add searching for tasks by ID/UUID
- Optimize serialization/deserialization process
- Implement a Ticketing system for intercommunication between the task issuer and task doer
