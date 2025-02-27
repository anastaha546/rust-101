mod models;
use models::{Task, TaskStatus};
use std::io::{stdin, stdout,self,Write};
use std::str::FromStr;
use rand::Rng;



fn main() {
        println!("+-------------------------------------------+");
        println!("|  🔥🔥  WELCOME TO TASK MANAGER   🔥🔥   |");
        println!("+-------------------------------------------+");
        println!("| Type 'help' to see available commands.    |");
        println!("+-------------------------------------------+");
    
        let mut tasks: Vec<Task> = Vec::new();  
    

        runprompt(&mut tasks);
    }


fn initialize_tasks(tasks: &mut Vec<Task>) {
    for _ in 1..=5 {
       new_task(tasks);
    }

    println!("Tasks added: {:?}", tasks);
}


fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        println!("Your tasks:");
        for task in tasks.iter() {
            println!("{}. {} - {:?} - {}", task.id, task.title, task.status, task.description);
        }
    }
}


fn mark_task_completed(tasks: &mut Vec<Task>) {
    view_tasks(tasks);

    let mut choice = String::new();
    println!("Enter the task number to mark as completed:");
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    if let Ok(index) = choice.trim().parse::<usize>() {
        if index > 0 && index <= tasks.len() {
            tasks[index - 1].status = TaskStatus::Completed; 
            println!("Task {} marked as completed!", index);
        } else {
            println!("Invalid task number.");
        }
    } else {
        println!("Please enter a valid number.");
    }
}


fn new_task(tasks: &mut Vec<Task>){
    let mut title = String::new();
    println!("Enter task title");
    io::stdin().read_line(&mut title).expect("Failed to read title");

    let status = loop {
        let mut status_input = String::new();
        println!("Enter task status");
        io::stdin().read_line(&mut status_input).expect("Failed to read status");

        match TaskStatus::from_str(status_input.trim()){
            Ok(valid_status) => break valid_status,
            Err(msg) => println!("{}",msg),
        }
    };

    let mut description = String::new();
    println!("Enter task description");
    io::stdin().read_line(&mut description).expect("Failed to read task description :( ");

    let new_task = Task {
        id:  rand::thread_rng().gen_range(100000..=999999),
        title: title.trim().to_string(),
        status,
        description
        };
    tasks.push(new_task);
}



fn runprompt(tasks: &mut Vec<Task>) {
    loop {
        let mut stdout = stdout();
        print!("(todo list) > ");
        stdout.flush().expect("Failed to flush stdout");

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Failed to read input");

        let args: Vec<&str> = buffer.trim().split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        match args[0] {
            "init" => initialize_tasks(tasks),
            "add" => new_task(tasks),
            "view" => view_tasks(tasks),
            "done" => mark_task_completed(tasks),
            "exit" => {
                println!("Exiting... 👋");
                break;
            }
            "help" => {
                println!("+-------------------------------------------+");
                println!("| Available Commands:                       |");
                println!("|  init    - Initialize task list 🚀       |");
                println!("|  add     - Add a new task 📝             |");
                println!("|  view    - View all tasks 📋             |");
                println!("|  done    - Mark a task as completed ✅   |");
                println!("|  exit    - Exit the task manager ❌      |");
                println!("+-------------------------------------------+");
            }
            _ => println!("Unknown command! Type 'help' for a list of commands."),
        }
    }
}
