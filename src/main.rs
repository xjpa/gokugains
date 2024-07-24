use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Exercise {
    type_: String,
    duration: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct DayEntry {
    date: String,
    exercises: Vec<Exercise>,
}

fn main() {
    let file_path = "exercises.json";
    loop {
        println!("\n===== GokuGains =====");
        println!("1. Add exercise");
        println!("2. View all exercises");
        println!("3. View summary report");
        println!("4. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        println!("\n");

        match choice.trim() {
            "1" => {
                add_exercise(file_path);
                pause();
            }
            "2" => {
                view_exercises(file_path);
                pause();
            }
            "3" => {
                view_summary(file_path);
                pause();
            }
            "4" => break,
            _ => {
                println!("Invalid option, please try again.");
                pause();
            }
        }
    }
}

fn add_exercise(file_path: &str) {
    print!("Enter exercise type: ");
    io::stdout().flush().unwrap();
    let mut exercise_type = String::new();
    io::stdin().read_line(&mut exercise_type).unwrap();

    print!("Enter duration (minutes): ");
    io::stdout().flush().unwrap();
    let mut duration = String::new();
    io::stdin().read_line(&mut duration).unwrap();
    let duration: u32 = duration.trim().parse().unwrap();

    let exercise = Exercise {
        type_: exercise_type.trim().to_string(),
        duration,
    };

    let mut entries = load_entries(file_path);
    let today = Local::now().format("%Y-%m-%d").to_string();

    if let Some(entry) = entries.iter_mut().find(|e| e.date == today) {
        entry.exercises.push(exercise);
    } else {
        entries.push(DayEntry {
            date: today,
            exercises: vec![exercise],
        });
    }

    save_entries(file_path, &entries);
    println!("Exercise added successfully!");
}

fn view_exercises(file_path: &str) {
    let entries = load_entries(file_path);
    for entry in entries {
        println!("Date: {}", entry.date);
        for exercise in entry.exercises {
            println!("  - {}: {} minutes", exercise.type_, exercise.duration);
        }
        println!();
    }
}

fn view_summary(file_path: &str) {
    let entries = load_entries(file_path);
    let mut total_exercises = 0;
    let mut total_duration = 0;
    let mut exercises_by_type = std::collections::HashMap::new();

    for entry in entries {
        for exercise in entry.exercises {
            total_exercises += 1;
            total_duration += exercise.duration;
            *exercises_by_type.entry(exercise.type_).or_insert(0) += 1;
        }
    }

    println!("Summary Report");
    println!("Total exercises: {}", total_exercises);
    println!("Total duration: {} minutes", total_duration);
    println!("Exercises by type:");
    for (type_, count) in exercises_by_type {
        println!("  - {}: {}", type_, count);
    }
}

fn load_entries(file_path: &str) -> Vec<DayEntry> {
    if !Path::new(file_path).exists() {
        return Vec::new();
    }

    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
}

fn save_entries(file_path: &str, entries: &[DayEntry]) {
    let json = serde_json::to_string_pretty(entries).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

fn pause() {
    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
}
