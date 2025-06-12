use std::env;

mod task;
mod utils;
use crate::task::actions::{add_new_task, delete_task, mark_task_as_done, print_task_list};
use crate::utils::utils::{load_tasks_from_file, save_task_to_file};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }

    let path = "todo_list.json";
    let mut tasks = load_tasks_from_file(&path);

    match args[1].to_lowercase().as_str() {
        "add" => {
            if let Some(description) = args.get(2) {
                add_new_task(&mut tasks, description);
                save_task_to_file(&path, &tasks);
            } else {
                print_help();
            }
        }
        "list" => {
            println!(" -- Current Tasks In List -- ");
            if tasks.len() > 0 {
                print_task_list(&tasks);
            } else {
                println!("There are currently no tasks");
            }
        }
        "done" => {
            if let Some(id) = args.get(2) {
                match id.parse() {
                    Ok(tid) => {
                        if tasks.len() > 0 {
                            mark_task_as_done(&mut tasks, tid);
                            save_task_to_file(&path, &tasks);
                        } else {
                            println!("There are currently no tasks");
                        }
                    }
                    Err(_e) => {
                        println!("Invalid task ID");
                    }
                }
            } else {
                print_help();
            }
        }
        "del" | "delete" => {
            if let Some(id) = args.get(2) {
                match id.parse() {
                    Ok(tid) => {
                        if tasks.len() > 0 {
                            delete_task(&mut tasks, tid);
                            save_task_to_file(&path, &tasks);
                        } else {
                            println!("There are currently no tasks");
                        }
                    }
                    Err(_e) => {
                        println!("Invalid task ID");
                    }
                }
            } else {
                print_help();
            }
        }
        _ => print_help(),
    }
}

fn print_help() {
    println!(" ------------------ How To Use ------------------ ");
    println!("Add '<description>' \t Add new task");
    println!("List \t\t\t Print all the tasks");
    println!("Done '<task id>' \t Mark the task as done");
    println!("Del '<task id>' \t Delete the task");
    println!(" ------------------------------------------------ ");
}
