use serde::{
    Serialize,
    Deserialize
};
use serde_json::{
    Value,
    from_str
};
use owo_colors::{
    OwoColorize,
    colors::css::{
        LimeGreen,
        Red,
        Orange,
        Yellow,
        Aqua,
        White,
    }
};
use chrono::NaiveDate;
use std::{
    env,
    path::PathBuf,
    fmt,
    io::{
        Read,
        Write,
        self,
        Seek
    },
    fs::{
        OpenOptions,
        create_dir_all
    },
    process::Command,
    thread,
    time::Duration,
    str::FromStr
};
use uuid::Uuid;

const FILE_PATH: &str = "db/tasks.json";

#[derive(Debug,Serialize,Deserialize,PartialEq)]
pub enum Status{
    Completed,
    NotCompleted,
}

impl std::fmt::Display for Status{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match *self{
            Status::Completed       => write!(f, "Completed"),
            Status::NotCompleted    => write!(f, "Not Completed"),
        }
    }
}

#[derive(Debug,Serialize,Deserialize,PartialEq)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

impl FromStr for Priority {
    type Err = ();

    fn from_str(input: &str) -> Result<Priority, Self::Err>{
        match input{
            "Critical"  => Ok(Priority::Critical),
            "High"      => Ok(Priority::High),
            "Medium"    => Ok(Priority::Medium),
            "Low"       => Ok(Priority::Low),
            _           => Err(()),
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Task{
    uuid:           Uuid,
    id:             i32,
    priority:       Priority,
    title:          String,
    description:    String,
    due_date:       NaiveDate,
    status:         Status,
    assignees:       Vec<String>,
}

impl Task{
    pub fn new(id:String, priority:String, title:String, description:String, due_date:String, assignees: Vec<String>) -> Task{
        let parsed_due_date = NaiveDate::parse_from_str(due_date.as_str(),"%Y-%m-%d").expect("Error Parsing Date");

        Task{
            uuid:           Uuid::new_v4(),
            id:             id.parse::<i32>().unwrap(),
            priority:       Priority::from_str(priority.as_str()).unwrap(),
            title:          title,
            description:    description,
            due_date:       parsed_due_date,
            status:         Status::NotCompleted,
            assignees:      assignees,
        }
    } 
}

pub fn read_input() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");

    return input.trim().to_string();
}

pub fn clear_screen(){
    let cmd_name = if cfg!(unix){
        "clear"
    } else if cfg!(windows){
        "cls"
    } else {
        panic!("Unsupported platform");
    };

    let status = Command::new(cmd_name).status().expect("Failed to execute command");

    if !status.success(){
        eprintln!("command failed with exit code: {}",status);
    }
}

pub fn prompt_user(message: &str) -> String {
    println!("{}", message);
    io::stdout().flush().unwrap();
    read_input()
}

pub fn main() {

    let exe_path = env::current_exe().expect("Unable to get current executable path");
    let mut dir_path: PathBuf = exe_path.parent().expect("Failed to get parent directory of executable").to_path_buf();

    if cfg!(target_os = "windows") {
        dir_path.push(FILE_PATH.replace("/", "\\"));
    } else {
        dir_path.push(FILE_PATH);
    }

    if let Some(parent) = dir_path.parent(){
        let _ = create_dir_all(parent);
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&dir_path)
        .expect("Unable to create file");

    let logo = r#"
░█▄█░█▀█░█▀▄░▀█▀░█▀█░█░░░░░█▀▄░█▀▀░█▄█░▀█▀░█▀█░█▀▄░█▀▀░█▀▄░░░█░█░▀█░░░░░▀▀▄░█░░
░█░█░█░█░█▀▄░░█░░█▀█░█░░░░░█▀▄░█▀▀░█░█░░█░░█░█░█░█░█▀▀░█▀▄░░░▀▄▀░░█░░░░░▄▀░░█▀▄
░▀░▀░▀▀▀░▀░▀░░▀░░▀░▀░▀▀▀░░░▀░▀░▀▀▀░▀░▀░▀▀▀░▀░▀░▀▀░░▀▀▀░▀░▀░░░░▀░░▀▀▀░▀░░▀▀▀░▀▀░
"#;

    //clear_screen();
    println!("{}", logo.bold().truecolor(176,0,0));
    println!("1. Add Task");
    println!("2. List Tasks");
    println!("3. Mark task as completed");
    println!("4. Remove task");
    println!("5. Edit a task");
    println!("6. Search for a task");
    println!("7. Exit");
    println!(); 
    loop {
        thread::sleep(Duration::from_secs(1));
        println!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        match choice{
            1 => {
                println!("Enter task details");
                let task_id = prompt_user("ID: ");
                let task_prio = prompt_user("Priority (Critical, High, Medium, Low): ");
                let task_title = prompt_user("Title: ");
                let task_desc = prompt_user("Description: ");
                let task_due_date = prompt_user("Due Date (YYYY-MM-DD Format): ");

                let mut assignees = Vec::new();
                println!("Enter at least 1 assignee name (type done to countinue): ");
                loop{
                    let assignee = read_input();
                    if assignee.to_lowercase() != "done"{
                        assignees.push(assignee);
                    } else if assignee.to_lowercase() == "done"{
                        break;
                    }
                }

                let ntask = Task::new(task_id, task_prio, task_title, task_desc, task_due_date, assignees);

                let mut contents = String::new();
                if let Err(err) = file.read_to_string(&mut contents) {
                    eprintln!("error reading file: {}", err);
                    return; 
                }

                let mut tasks: Vec<Task> = match serde_json::from_str(&contents) {
                    Ok(tasks)   => tasks,
                    Err(_)      => Vec::new(),
                };

                tasks.push(ntask);

                let updated_json = match serde_json::to_string_pretty(&tasks) {
                    Ok(json) => json,
                    Err(err) => {
                        eprintln!("Error serializing tasks: {}", err);
                        return;
                    }
                };
                
                file.seek(std::io::SeekFrom::Start(0)).expect("Unable to seek start of the file");

                if let Err(err) = file.write_all(updated_json.as_bytes()) {
                    eprintln!("Error writing to file: {}", err);
                    return;
                }
            }
            2 => {

                file.seek(std::io::SeekFrom::Start(0)).expect("Unable to seek to the beginning of the file");

                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Unable to read file");

                let json_content: Value = from_str(&contents).expect("Unable to parse");
                println!("{}","-".repeat(110));
                if let Some(tasks) = json_content.as_array() {
                    for task in tasks {
                        if let Some(task_obj) = task.as_object() {
                            if let Some(id) = task_obj.get("id") {
                                println!("{}: {}",r#"ID"#.underline().bold().fg::<Aqua>() ,id.bold().fg::<White>());
                            }
                                
                            if let Some(uuid) = task_obj.get("uuid") {
                                println!("{} : {}",r#"UUID"#.underline().bold().fg::<Aqua>(),uuid);
                            } 

                            if let Some(priority) = task_obj.get("priority") {
                                match priority.as_str() {
                                    Some("Critical")    => println!("{}: {}",r#"Priority"#.underline().bold().fg::<Aqua>(),r#"Critical"#.bold().fg::<Red>()),
                                    Some("High")        => println!("{}: {}",r#"Priority"#.underline().bold().fg::<Aqua>(),r#"High"#.bold().fg::<Orange>()),
                                    Some("Medium")      => println!("{}: {}",r#"Priority"#.underline().bold().fg::<Aqua>(),r#"Medium"#.bold().fg::<Yellow>()),
                                    Some("Low")         => println!("{}: {}",r#"Priority"#.underline().bold().fg::<Aqua>(),r#"Low"#.bold().fg::<LimeGreen>()),
                                    _                   => println!("Not a Task Priority"),
                                }
                            }

                            if let Some(title) = task_obj.get("title") {
                                println!("{}: {}",r#"Title"#.underline().bold().fg::<Aqua>() , title);
                            }

                            if let Some(description) = task_obj.get("description") {
                                println!("{}: {}",r#"Description"#.underline().bold().fg::<Aqua>() ,description);
                            }

                            if let Some(due_date) = task_obj.get("due_date") {
                                println!("{}: {}",r#"Due Date"#.underline().bold().fg::<Aqua>() , due_date);
                            }

                            if let Some(status) = task_obj.get("status") {
                                println!("{}: {}",r#"Status"#.underline().bold().fg::<Aqua>(), status.bold());
                            }

                            if let Some(assignees) = task_obj.get("assignees") {
                                println!("{}: {}",r#"Assignees"#.underline().bold().fg::<Aqua>(), assignees.to_string().trim_matches(|br| br == '[' || br == ']'));
                            }

                            println!("{}","-".repeat(110));
                        } else {
                            println!("Task is not an object");
                        }
                    }
                } else {
                    println!("JSON content does not represent an array of tasks");
                }
            }
            3 => {
                file.seek(std::io::SeekFrom::Start(0)).expect("Unable to seek to the start of the file");

                println!("Enter the ID of the task to mark as completed: ");
                let init_id: u32 = read_input().trim().parse().expect("unable to parse to int");
                
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Unable to read file");
                let mut tasks: Value = serde_json::from_str(&contents).expect("Unable to deserialize");

                if let Some(task) = tasks.as_array_mut().and_then(
                    |arr| {
                    arr.iter_mut().find(
                            |t| t.get("id").and_then(
                                |id| id.as_u64()) == Some(init_id.into()))
                }) {
                    if let Some(status) = task.get_mut("status") {
                        *status = Some(Status::Completed.to_string()).into();

                        file.seek(std::io::SeekFrom::Start(0)).expect("Unable to seek to the start of the file");
                        file.set_len(0).expect("Unable to truncate file");

                        let updated_json = serde_json::to_string_pretty(&tasks).expect("Unable to serialize");
                        file.write_all(&updated_json.as_bytes()).expect("Unable to write to file");
                    } else {
                        println!("Task with id: {} not found",init_id);
                    }
                }
            }
            4 => {
                file.seek(std::io::SeekFrom::Start(0)).expect("Unable to seek to start of the file");

                println!("Enter the ID of the task you wish to remove: ");
                let init_id: u32 = read_input().trim().parse().expect("Unable to parse to int");

                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Unable to read file");

                let mut tasks: Value = serde_json::from_str(&contents).expect("Unable to deserialize");

                if let Some(index) = tasks.as_array_mut().and_then(|arr| {
                    arr.iter().position(|t| t.get("id").and_then(|id| id.as_u64()) == Some(init_id.into()))
                }) {
                    let matching_tasks: Vec<&Value> = tasks.as_array().unwrap().iter().filter(|t| {
                        t.get("id").and_then(|id| id.as_u64()) == Some(init_id.into())
                    }).collect();

                    if matching_tasks.len() > 1 {
                        println!("Multiple tasks found with ID: {}", init_id);
                        println!("Please select the number corresponding to the UUID of the task you wish to remove:");

                        for (index, task) in matching_tasks.iter().enumerate() {
                            if let Some(uuid) = task.get("uuid").and_then(|uuid| uuid.as_str()) {
                                println!("{}. {}", index + 1, uuid);
                            }
                        }

                        println!("Enter the number corresponding to the UUID to delete: ");
                        let selected_index: usize = read_input().trim().parse().expect("Unable to parse selection");

                        if selected_index > 0 && selected_index <= matching_tasks.len() {
                            let task_to_remove = matching_tasks[selected_index - 1].clone();
                            let uuid_to_remove = task_to_remove.get("uuid").and_then(|uuid| uuid.as_str()).unwrap();

                            if let Some(task) = tasks.as_array_mut() {
                                if let Some(index) = task.iter().position(|t| {
                                    t.get("uuid").and_then(|uuid| uuid.as_str()) == Some(uuid_to_remove)
                                }) {
                                    task.remove(index);
                                    println!("Task with UUID: {} deleted successfully", uuid_to_remove);
                                }
                            }
                        } else {
                            println!("Invalid selection.");
                        }
                    } else {
                        if let Some(task) = tasks.as_array_mut() {
                            task.remove(index);
                            println!("Task with ID: {} deleted successfully", init_id);
                        }
                    }

                    file.seek(std::io::SeekFrom::Start(0)).expect("Unable to seek to start of the file");
                    file.set_len(0).expect("Unable to truncate file");

                    let updated_json = serde_json::to_string_pretty(&tasks).expect("Unable to serialize");
                    file.write_all(updated_json.as_bytes()).expect("Unable to write to file");
                } else {
                    println!("Task with ID: {} not found", init_id);
                }
            }
            5 => {
                file.seek(std::io::SeekFrom::Start(0)).expect("Unable to seek to start of the file");

                println!("Enter the ID of the task you wish to edit: ");
                let init_id: u32 = read_input().trim().parse().expect("Unable to parse to int");

                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Unable to read file");

                let mut tasks: Value = serde_json::from_str(&contents).expect("Unable to deserialize");

                if let Some(task) = tasks.as_array_mut().and_then(|arr| {
                    arr.iter_mut().find(|t| t.get("id").and_then(|id| id.as_u64()) == Some(init_id.into()))
                }) {
                    println!("Enter the field you wish to edit (id, priority, title, description, due_date, status, assignees): ");
                    let field = read_input();

                    match field.as_str() {
                        "id" => {
                            println!("Enter the new ID: ");
                            let new_id: i32 = read_input().trim().parse().expect("Unable to parse to int");
                            task.as_object_mut().unwrap().insert("id".to_string(), new_id.into());
                        }
                        "priority" => {
                            println!("Enter the new priority (Critical, High, Medium, Low): ");
                            let new_priority = read_input();
                            task.as_object_mut().unwrap().insert("priority".to_string(), new_priority.into());
                        }
                        "title" => {
                            println!("Enter the new title: ");
                            let new_title = read_input();
                            task.as_object_mut().unwrap().insert("title".to_string(), new_title.into());
                        }
                        "description" => {
                            println!("Enter the new description: ");
                            let new_description = read_input();
                            task.as_object_mut().unwrap().insert("description".to_string(), new_description.into());
                        }
                        "due_date" => {
                            println!("Enter the new due date (YYYY-MM-DD Format): ");
                            let new_due_date = read_input();
                            task.as_object_mut().unwrap().insert("due_date".to_string(), new_due_date.into());
                        }
                        "status" => {
                            println!("Enter the new status (Completed, NotCompleted): ");
                            let new_status = read_input();
                            task.as_object_mut().unwrap().insert("status".to_string(), new_status.into());
                        }
                        "assignees" => {
                            println!("Enter the new assignees (separated by commas): ");
                            let new_assignees = read_input();
                            task.as_object_mut().unwrap().insert("assignees".to_string(), new_assignees.into());
                        }
                        _ => {
                            println!("Invalid field");
                        }
                    }
                    let updated_json = serde_json::to_string_pretty(&tasks).expect("Unable to serialize");
                    file.seek(std::io::SeekFrom::Start(0)).expect("Unable to seek to start of the file");
                    file.set_len(0).expect("Unable to truncate file");
                    file.write_all(updated_json.as_bytes()).expect("Unable to write to file");
                } else {
                    println!("Task with ID: {} not found", init_id);
                }
            }
            6 => {
                file.seek(std::io::SeekFrom::Start(0)).expect("Unable to seek to start of the file");

                println!("Enter the field you wish to search by (id, priority, title, description, due_date, status, assignees): ");
                let field = read_input();

                println!("Enter the value you wish to search for: ");
                let value = read_input();

                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Unable to read file");

                let tasks: Value = serde_json::from_str(&contents).expect("Unable to deserialize");

                let matching_tasks: Vec<&Value> = tasks.as_array().unwrap().iter().filter(|t| {
                    if let Some(task) = t.as_object() {
                        match field.as_str() {
                            "id" => task.get("id").and_then(|id| id.as_u64()) == Some(value.parse().unwrap()),
                            "priority" => task.get("priority").and_then(|priority| priority.as_str()) == Some(value.as_str()),
                            "title" => task.get("title").and_then(|title| title.as_str()) == Some(value.as_str()),
                            "description" => task.get("description").and_then(|description| description.as_str()) == Some(value.as_str()),
                            "due_date" => task.get("due_date").and_then(|due_date| due_date.as_str()) == Some(value.as_str()),
                            "status" => task.get("status").and_then(|status| status.as_str()) == Some(value.as_str()),
                            "assignees" => task.get("assignees").and_then(|assignees| assignees.as_array()).map(|assignees| {
                                assignees.iter().any(|a| a.as_str() == Some(value.as_str()))
                            }).unwrap_or(false),
                            _ => false,
                        }
                    } else {
                        false
                    }
                }).collect();

                if matching_tasks.is_empty() {
                    println!("No tasks found with {} equal to {}", field, value);
                } else {
                    println!("{} tasks found with {} equal to {}", matching_tasks.len(), field, value);
                    for task in matching_tasks {
                        println!("{}", serde_json::to_string_pretty(task).expect("Unable to serialize"));
                    }
                }
            }
            7 => {
                println!("{}",r#"Exiting..."#.bold().red());
                let duration = Duration::from_secs(1);
                thread::sleep(duration);
                break;
            }
            _ => {
                println!("Invalid option, please choose a number between 1&5.");
            }
        }
    }
}
