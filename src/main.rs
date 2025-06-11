use rand::Rng;
use serde::{Deserialize, Serialize};

use std::path::Path;
use std::{env, fs};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: i64,
    description: String,
    done: bool,
}

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

fn add_new_task(tasks: &mut Vec<Task>, description: &String) {
    tasks.push(Task {
        id: generate_random_integer(1000000, 9999999),
        description: description.clone(),
        done: false,
    });
}

fn print_task_list(tasks: &Vec<Task>) {
    for t in tasks {
        let status = if t.done { "[✔]" } else { "[ ]" };
        println!("{}:{} - {}", t.id, status, t.description);
    }
}

fn mark_task_as_done(tasks: &mut Vec<Task>, id: i64) {
    for t in tasks {
        if t.id == id {
            t.done = true;
            return;
        }
    }
}

fn delete_task(tasks: &mut Vec<Task>, id: i64) {
    for (i, t) in tasks.iter().enumerate() {
        if t.id == id {
            tasks.remove(i);
            return;
        }
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

fn load_tasks_from_file(path: &str) -> Vec<Task> {
    if !Path::new(path).exists() {
        return Vec::new();
    }

    let data = fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

fn save_task_to_file(path: &str, tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(path, data).unwrap();
}

/// This function generates a random integer between two integers (inclusive)
///
/// # Parameters
/// - `min`: The floor of the random integer
/// - `max`: The ceiling of the random integer
fn generate_random_integer(min: i64, max: i64) -> i64 {
    let mut rng = rand::rng();
    let n: i64 = rng.random_range(min..=max);
    return n;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_new_task() {
        let mut tasks = vec![];

        let test1_description = String::from("Test 1 Description");
        add_new_task(&mut tasks, &test1_description);
        assert_eq!(tasks.len(), 1);
        assert!(!tasks[0].done);
    }

    #[test]
    fn test_mark_task_as_done() {
        let mut tasks: Vec<Task> = Vec::new();

        let id = 123;
        tasks.push(Task {
            id: id,
            description: "Task Description".to_string(),
            done: false,
        });

        mark_task_as_done(&mut tasks, id);
        assert!(tasks[0].done);
    }

    #[test]
    fn test_delete_task() {
        let mut tasks: Vec<Task> = Vec::new();

        let id = 123;
        tasks.push(Task {
            id: id,
            description: "Task Description".to_string(),
            done: false,
        });

        assert_eq!(tasks.len(), 1);
        delete_task(&mut tasks, id);
        assert_eq!(tasks.len(), 0);
    }
}
