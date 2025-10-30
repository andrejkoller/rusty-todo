use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    done: bool,
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks();

    loop {
        println!("\n 🦀 RUSTY TODO 🦀");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Mark Task as Done");
        println!("4. Delete Task");
        println!("5. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => add_task(&mut tasks),
            "2" => list_tasks(&tasks),
            "3" => mark_task_as_done(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                save_tasks(&tasks);
                println!("💾 Tasks saved! Goodbye! 👋");
                break;
            }
            _ => println!("❌ Invalid option, please try again."),
        }
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    let mut description = String::new();
    println!("Enter task description:");
    io::stdin().read_line(&mut description).unwrap();

    let next_id = if let Some(last_task) = tasks.last() {
        last_task.id + 1
    } else {
        1
    };

    let task = Task {
        id: next_id,
        description: description.trim().to_string(),
        done: false,
    };

    tasks.push(task);
    println!("✅ Task added!");
}

fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("📭 No tasks available.");
        return;
    }

    println!("📝 Your Tasks:");
    for task in tasks {
        println!("{}: {}", task.id, task.description);
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    list_tasks(tasks);

    println!("Enter the task number to delete:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= tasks.len() {
            tasks.remove(index - 1);
            println!("🗑️ Task deleted!");
        } else {
            println!("❌ Invalid task number.");
        }
    } else {
        println!("❌ Please enter a valid number.");
    }
}

fn mark_task_as_done(tasks: &mut Vec<Task>) {
    list_tasks(tasks);

    println!("Enter the task number to mark as done:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= tasks.len() {
            tasks[index - 1].done = true;
            tasks[index - 1].description = format!("{} ✅", tasks[index - 1].description);
            println!("✅ Task marked as done!");
        } else {
            println!("❌ Invalid task number.");
        }
    } else {
        println!("❌ Please enter a valid number.");
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).unwrap();
    fs::write("tasks.json", json).unwrap();
}

fn load_tasks() -> Vec<Task> {
    if let Ok(data) = fs::read_to_string("tasks.json") {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}
