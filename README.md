Task Manager CLI

This Rust program provides a command-line interface (CLI) for managing tasks efficiently. Users can add, list, mark as completed, and remove tasks seamlessly.
Features:

    Serialization and Deserialization: Utilizes serde and serde_json for efficient data serialization and deserialization of task data.
    Command-line Styling: Enhances user experience with colorful and styled CLI interface using owo_colors.
    Date and Time Handling: Incorporates chrono for precise handling of dates and times in task management.
    UUID Generation: Employs uuid to generate universally unique identifiers (UUIDs) for tasks.
    Main Components:
        Task Structs and Enums: Defines structures and enums for tasks, statuses, and priorities.
        User Interaction Functions: Implements functions for user input, console screen clearing, and user prompts.
        File Management: Manages tasks stored in a JSON file (db/tasks.json), enabling read, write, and update operations.
        Main Function: Orchestrates the CLI functionality, offering a menu for users to add, list, mark, or remove tasks.
        Conditional Logic: Handles file paths based on the operating system using platform-specific conditional compilation.
        Error Handling: Ensures robustness with error handling for file operations, JSON parsing, and user input.

The program simplifies task management from the command line, providing essential features for efficient task organization and tracking.
