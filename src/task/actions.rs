use crate::task::model::Task;
use crate::utils::utils::generate_random_integer;

pub fn add_new_task(tasks: &mut Vec<Task>, description: &String) {
    tasks.push(Task {
        id: generate_random_integer(1000000, 9999999),
        description: description.clone(),
        done: false,
    });
}

pub fn print_task_list(tasks: &Vec<Task>) {
    for t in tasks {
        let status = if t.done { "[✔]" } else { "[ ]" };
        println!("{}:{} - {}", t.id, status, t.description);
    }
}

pub fn mark_task_as_done(tasks: &mut Vec<Task>, id: i64) {
    for t in tasks {
        if t.id == id {
            t.done = true;
            return;
        }
    }
}

pub fn delete_task(tasks: &mut Vec<Task>, id: i64) {
    for (i, t) in tasks.iter().enumerate() {
        if t.id == id {
            tasks.remove(i);
            return;
        }
    }
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
