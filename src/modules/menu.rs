use crate::modules::storage::{load_tasks, save_tasks};
use crate::modules::task::Task;
use std::io;

pub fn run_menu() {
    let mut tasks = load_tasks();

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
            "3" => mark_done(&mut tasks),
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

    let next_id = tasks.last().map_or(1, |t| t.id + 1);
    tasks.push(Task::new(next_id, description.trim().to_string()));
    println!("✅ Task added!");
}

fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("📭 No tasks available.");
        return;
    }

    println!("📝 Your Tasks:");

    for task in tasks {
        let status = if task.done { "✔️" } else { "⏳" };
        println!("{}: {} {}", task.id, task.description, status);
    }
}

fn mark_done(tasks: &mut Vec<Task>) {
    list_tasks(tasks);

    println!("Enter the task number to mark as done:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(id) = input.trim().parse::<u32>() {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.mark_done();
            println!("✅ Task marked as done!");
        } else {
            println!("❌ Invalid task number.");
        }
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    list_tasks(tasks);

    println!("Enter the task number to delete:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(index) = input.trim().parse::<u32>() {
        if let Some(pos) = tasks.iter().position(|t| t.id as u32 == index) {
            tasks.remove(pos);
            println!("🗑️ Task deleted!");
        } else {
            println!("❌ Invalid task number.");
        }
    }
}
