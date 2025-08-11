use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    name: String,
    completed: bool,
}

const FILE_NAME: &str = "tasks.json";

fn add_task(todo: &mut Vec<Task>) {
    let mut task_name = String::from("");
    println!("Enter your Task:");
    io::stdin()
        .read_line(&mut task_name)
        .expect("Failed to read inputs");

    todo.push(Task {
        name: task_name.trim().to_string(),
        completed: false,
    });
}

fn update_task(todo: &mut Vec<Task>) {
    let mut user_input = String::from("");
    println!("What task you want to update..");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read inputs");
    let requested_todo_number: usize = user_input.trim().parse().expect("Invalid input");

    if requested_todo_number > todo.len() {
        println!("Invalid requested Task Number");
        return;
    }
    user_input.clear();
    println!("task {requested_todo_number}'s name ");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to inputs");
    let updated_name = user_input.trim().to_string();

    user_input.clear();

    println!("task {requested_todo_number}'s status ");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to inputs");
    let updated_task_status = user_input.trim().to_lowercase();
    user_input.clear();

    let requested_task = todo.get(requested_todo_number - 1);

    if let Some(data) = requested_task {
        //checking if user updated task name with empty so it will take default prev one
        todo[requested_todo_number - 1].name = if updated_name.is_empty() {
            data.name.to_string()
        } else {
            updated_name
        };

        todo[requested_todo_number - 1].completed = if updated_task_status.is_empty() {
            false
        } else {
            updated_task_status.parse().unwrap_or(false)
        }
    } else {
        println!("Invalid requested Task Number");
    }
}

fn remove_task(todo: &mut Vec<Task>) {
    let mut user_input = String::from("");
    println!("What task you want to delete..");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read inputs");
    let requested_todo_number: usize = user_input.trim().parse().expect("Invalid input");
    user_input.clear();

    if requested_todo_number > todo.len() {
        println!("Invalid requested Task Number");
    } else {
        todo.remove(requested_todo_number - 1);
    }
}

fn show_task(todo: &Vec<Task>) {
    for (i, v) in todo.iter().enumerate() {
        let task_status = if v.completed { "✅" } else { "❌" };
        let updated_str = format!("{}. [{}] {}", i + 1, task_status, v.name);
        println!("{updated_str}");
    }
}

fn read_tasks(todo: &mut Vec<Task>) {
    let res = fs::read_to_string(FILE_NAME);
    let data = match res {
        Ok(data) => data,
        Err(_) => String::from(""),
    };
    let deserialized_tasks: Result<Vec<Task>, serde_json::Error> = serde_json::from_str(&data);
    // todo.append(&mut deserialized_tasks);
    if let Ok(mut data) = deserialized_tasks {
        todo.append(&mut data)
    }
}

fn save_task(todo: &Vec<Task>) {
    let res = serde_json::to_string(todo).unwrap();
    let res = fs::write(FILE_NAME, res);

    match res {
        Ok(_) => println!("SuccesFully Saved tasks Data."),
        Err(_) => println!("Failed to save tasks data."),
    }
}

fn clear_all_task(todo: &mut Vec<Task>) {
    todo.clear();
    println!("All Tasks are cleared.");
}
fn main() {
    let mut todo: Vec<Task> = Vec::new();

    read_tasks(&mut todo);

    while true {
        let mut user_input = String::from("");
        println!("-------------------------------------------------");
        println!("What operation you want to perform ..\n");
        println!(
            "1)Add Task\n 2)Update Task \n 3)Remove Task \n 4)show task \n 5)Save Tasks \n 6)Clear All Tasks \n 7)Exit"
        );
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read inputs");

        let operation: u8 = user_input.trim().parse().unwrap_or(8);

        if operation == 1 {
            add_task(&mut todo);
        } else if operation == 2 {
            update_task(&mut todo);
        } else if operation == 3 {
            remove_task(&mut todo);
        } else if operation == 4 {
            if todo.len() <= 0 {
                println!("No Task exist !");
            } else {
                show_task(&todo);
            }
        } else if operation == 5 {
            save_task(&todo);
        } else if operation == 6 {
            clear_all_task(&mut todo);
        } else if operation == 7 {
            break;
        } else {
            println!("Wrong operation [❌]...");
        }
    }
}
