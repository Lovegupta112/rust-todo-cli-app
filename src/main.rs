use std::io;

#[derive(Debug)]
struct Task {
    name: String,
    completed: bool,
}

fn add_task(todo: &mut Vec<Task>)  {
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

fn update_task(todo: &mut Vec<Task>){
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

        todo[requested_todo_number -1 ].completed = if updated_task_status.is_empty() {
             false
        }
        else {
            updated_task_status.parse().unwrap_or(false)
        }

    } else {
        println!("Invalid requested Task Number");
    }
}

fn remove_task(todo: &mut Vec<Task>)  {
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

fn main() {
    let mut todo: Vec<Task> = Vec::new();


    while true {
         let mut user_input = String::from("");
          println!("-------------------------------------------------");
          println!("What operation you want to perform .. ");
          println!("1)Add Task , 2)Update Task, 3)Remove Task, 4)show task,  5)Exit");
         io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read inputs");

         let operation: u8=user_input.trim().parse().unwrap_or(5);

         if operation == 1 {
            add_task(&mut todo);
         }
         else if operation ==2 {
              update_task(&mut todo);
         }
         else if operation ==3 {
            remove_task(&mut todo);
         }
         else if  operation ==4  {
             println!("All Tasks: {:#?}",todo);
         }
         else if operation ==5 {
              break;
         }
         else {
            println!("Wrong operation ...");
         }
    }
    
}
