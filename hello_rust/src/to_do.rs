use std::fs;
use std::io::{self,Write};
use serde::{Serialize,Deserialize};

const FILE_PATH:&str = "tasks.json";

#[derive(Serialize,Deserialize,Debug)]
struct Task{
    id:u32,
    title:String,
}

fn load_tasks()->Vec<Task>{
    let data = fs::read_to_string(FILE_PATH);
    match data{
        Ok(content)=>serde_json::from_str(&content).unwrap_or(Vec::new()),
        Err(_)=>Vec::new(),
    }
}

fn save_tasks(tasks: &Vec<Task>){
    let data=serde_json::to_string_pretty(tasks).unwrap();
    fs::write(FILE_PATH,data).expect("Unable to write file");
}
fn add_task(tasks: &mut Vec<Task>){
    let mut title = String::new();
    print!("Enter task: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut title).unwrap();
    let id = tasks.len() as u32 + 1;
    tasks.push(Task{
        id,
        title: title.trim().to_string(),
    });
    println!("Task added!");
}

fn view_tasks(tasks: &Vec<Task>){
    if tasks.is_empty(){
        println!("No tasks found.");
        return;
    }
    println!("\nYour Tasks");
    for task in tasks{
        println!("{}: {}",task.id, task.title);
    }
}

fn delete_task(task: &mut Vec<Task>){
    let mut input = String::new();
    println!("Enter task ID to delete: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let id:u32 = input.trim().parse().unwrap_or(0);
    let initial_len = task.len();
    task.retain(|task| task.id!=id);
    if task.len()<initial_len{
        println!("Taskk deleted!");
    }
    else{
        println!("Task not found.");
    }
}

pub fn main(){
    let  mut tasks = load_tasks();
    loop{
        println!("\n ==== To-Do App ====");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Delete Task");
        println!("4. Exit");
        let mut choice =String::new();
        print!("Enter Choice: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim(){
            "1"=>{
                add_task(&mut tasks);
                save_tasks(&tasks);
            }
            "2" => {
                view_tasks(&tasks);
            }
            "3"=>{
                delete_task(&mut tasks);
                save_tasks(&tasks);
            }
            "4"=>{
                println!("Exiting...");
                break;
            }
            _=> println!("Invalid choice"),
        }
    }
}