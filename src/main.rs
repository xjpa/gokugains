use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Exercise {
    type_: String,
    sets: u32,
    reps: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DayEntry {
    date: String,
    exercises: Vec<Exercise>,
}

fn main() -> io::Result<()> {
    let file_path = "exercises.json";
    loop {
        println!("\n===== GokuGains =====");
        println!("1. Add exercise");
        println!("2. View all exercises");
        println!("3. View summary report");
        println!("4. Edit exercise");
        println!("5. Delete exercise");
        println!("6. Exit");
        print!("Choose an option: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        println!("\n");

        match choice.trim() {
            "1" => {
                if let Err(e) = add_exercise(file_path) {
                    println!("Error adding exercise: {}", e);
                }
            }
            "2" => {
                if let Err(e) = view_exercises(file_path) {
                    println!("Error viewing exercises: {}", e);
                }
            }
            "3" => {
                if let Err(e) = view_summary(file_path) {
                    println!("Error viewing summary: {}", e);
                }
            }
            "4" => {
                if let Err(e) = edit_exercise(file_path) {
                    println!("Error editing exercise: {}", e);
                }
            }
            "5" => {
                if let Err(e) = delete_exercise(file_path) {
                    println!("Error deleting exercise: {}", e);
                }
            }
            "6" => break,
            _ => println!("Invalid option, please try again."),
        }
        pause();
    }
    Ok(())
}

fn add_exercise(file_path: &str) -> io::Result<()> {
    print!("Enter exercise type: ");
    io::stdout().flush()?;
    let mut exercise_type = String::new();
    io::stdin().read_line(&mut exercise_type)?;

    let sets = get_positive_integer("Enter number of sets: ")?;
    let reps = get_positive_integer("Enter number of reps per set: ")?;

    let exercise = Exercise {
        type_: exercise_type.trim().to_string(),
        sets,
        reps,
    };

    let mut entries = load_entries(file_path)?;
    let today = Local::now().format("%Y-%m-%d").to_string();

    if let Some(entry) = entries.iter_mut().find(|e| e.date == today) {
        entry.exercises.push(exercise);
    } else {
        entries.push(DayEntry {
            date: today,
            exercises: vec![exercise],
        });
    }

    save_entries(file_path, &entries)?;
    println!("Exercise added successfully!");
    Ok(())
}

fn edit_exercise(file_path: &str) -> io::Result<()> {
    let mut entries = load_entries(file_path)?;

    println!("Select the date of the exercise you want to edit:");
    for (i, entry) in entries.iter().enumerate() {
        println!("{}. {}", i + 1, entry.date);
    }

    let date_index = get_index_input(entries.len())? - 1;

    println!("Select the exercise you want to edit:");
    for (i, exercise) in entries[date_index].exercises.iter().enumerate() {
        println!(
            "{}. {} - {} sets x {} reps",
            i + 1,
            exercise.type_,
            exercise.sets,
            exercise.reps
        );
    }

    let exercise_index = get_index_input(entries[date_index].exercises.len())? - 1;

    let exercise = &mut entries[date_index].exercises[exercise_index];

    println!(
        "Editing: {} - {} sets x {} reps",
        exercise.type_, exercise.sets, exercise.reps
    );

    print!("Enter new exercise type (or press Enter to keep current): ");
    io::stdout().flush()?;
    let mut new_type = String::new();
    io::stdin().read_line(&mut new_type)?;
    if !new_type.trim().is_empty() {
        exercise.type_ = new_type.trim().to_string();
    }

    print!("Enter new number of sets (or press Enter to keep current): ");
    io::stdout().flush()?;
    let mut new_sets = String::new();
    io::stdin().read_line(&mut new_sets)?;
    if !new_sets.trim().is_empty() {
        exercise.sets = new_sets
            .trim()
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid number"))?;
    }

    print!("Enter new number of reps (or press Enter to keep current): ");
    io::stdout().flush()?;
    let mut new_reps = String::new();
    io::stdin().read_line(&mut new_reps)?;
    if !new_reps.trim().is_empty() {
        exercise.reps = new_reps
            .trim()
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid number"))?;
    }

    save_entries(file_path, &entries)?;
    println!("Exercise updated successfully!");
    Ok(())
}

fn delete_exercise(file_path: &str) -> io::Result<()> {
    let mut entries = load_entries(file_path)?;

    println!("Select the date of the exercise you want to delete:");
    for (i, entry) in entries.iter().enumerate() {
        println!("{}. {}", i + 1, entry.date);
    }

    let date_index = get_index_input(entries.len())? - 1;

    println!("Select the exercise you want to delete:");
    for (i, exercise) in entries[date_index].exercises.iter().enumerate() {
        println!(
            "{}. {} - {} sets x {} reps",
            i + 1,
            exercise.type_,
            exercise.sets,
            exercise.reps
        );
    }

    let exercise_index = get_index_input(entries[date_index].exercises.len())? - 1;

    entries[date_index].exercises.remove(exercise_index);

    if entries[date_index].exercises.is_empty() {
        entries.remove(date_index);
    }

    save_entries(file_path, &entries)?;
    println!("Exercise deleted successfully!");
    Ok(())
}

fn get_index_input(max: usize) -> io::Result<usize> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().parse() {
            Ok(num) if num > 0 && num <= max => return Ok(num),
            _ => println!("Please enter a number between 1 and {}.", max),
        }
    }
}

fn get_positive_integer(prompt: &str) -> io::Result<u32> {
    loop {
        print!("{}", prompt);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().parse() {
            Ok(num) if num > 0 => return Ok(num),
            _ => println!("Please enter a positive integer."),
        }
    }
}

fn view_exercises(file_path: &str) -> io::Result<()> {
    let entries = load_entries(file_path)?;
    for entry in entries {
        println!("Date: {}", entry.date);
        for exercise in entry.exercises {
            println!(
                "  - {}: {} sets x {} reps",
                exercise.type_, exercise.sets, exercise.reps
            );
        }
        println!();
    }
    Ok(())
}

fn view_summary(file_path: &str) -> io::Result<()> {
    let entries = load_entries(file_path)?;
    let mut total_exercises = 0;
    let mut total_sets = 0;
    let mut total_reps = 0;
    let mut exercises_by_type = std::collections::HashMap::new();

    for entry in entries {
        for exercise in entry.exercises {
            total_exercises += 1;
            total_sets += exercise.sets;
            total_reps += exercise.sets * exercise.reps;
            exercises_by_type
                .entry(exercise.type_)
                .and_modify(|e: &mut (u32, u32)| {
                    e.0 += exercise.sets;
                    e.1 += exercise.sets * exercise.reps;
                })
                .or_insert((exercise.sets, exercise.sets * exercise.reps));
        }
    }

    println!("Summary Report");
    println!("Total exercises logged: {}", total_exercises);
    println!("Total sets: {}", total_sets);
    println!("Total reps: {}", total_reps);
    println!("Exercises by type:");
    for (type_, (sets, reps)) in exercises_by_type {
        println!("  - {}: {} sets, {} total reps", type_, sets, reps);
    }
    Ok(())
}

fn load_entries(file_path: &str) -> io::Result<Vec<DayEntry>> {
    if !Path::new(file_path).exists() {
        return Ok(Vec::new());
    }

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn save_entries(file_path: &str, entries: &[DayEntry]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(entries)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;
    file.write_all(json.as_bytes())
}

fn pause() {
    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    let _ = io::stdin().read_line(&mut _input);
}
