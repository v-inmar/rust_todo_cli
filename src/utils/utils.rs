use crate::task::model::Task;
use rand::Rng;
use std::fs;
use std::path::Path;

/// This function generates a random integer between two integers (inclusive)
///
/// # Parameters
/// - `min`: The floor of the random integer
/// - `max`: The ceiling of the random integer
pub fn generate_random_integer(min: i64, max: i64) -> i64 {
    let mut rng = rand::rng();
    let n: i64 = rng.random_range(min..=max);
    return n;
}

pub fn load_tasks_from_file(path: &str) -> Vec<Task> {
    if !Path::new(path).exists() {
        return Vec::new();
    }

    let data = fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

pub fn save_task_to_file(path: &str, tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(path, data).unwrap();
}
