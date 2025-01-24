use std::fs::{self, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    loop {
        let mut action = String::new();
        println!("\nWhat do you want to do? (add, list, remove or break): ");

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        let action = action.trim();

        if action == "break" {
            break;
        }

        match action.as_ref() {
            "add" => {
                if let Err(e) = manage_file() {
                    eprint!("Error {}", e)
                }
            }
            "list" => {
                if let Err(e) = read_file() {
                    eprint!("Error {}", e)
                }
            }
            "remove" => {
                if let Err(e) = remove_all() {
                    eprint!("Error {}", e)
                }
            }
            _ => println!("No valid option"),
        }
    }
}

fn create_file() -> io::Result<()> {
    // Create file if is not exist
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("tasks.txt")?;
    // println!("\nFile created!");
    append_to_file()?;
    Ok(())
}

fn append_to_file() -> io::Result<()> {
    println!("\nPlease enter a text: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut file = OpenOptions::new()
        .write(true)
        .append(true) // Write at the last line
        .open("tasks.txt")?;

    file.write_all(format!("{}\n", input.trim()).as_bytes())?;
    // println!("\nText added to the file!");

    Ok(())
}

fn read_file() -> io::Result<()> {
    if Path::new("tasks.txt").exists() {
        OpenOptions::new().read(true).open("tasks.txt")?;
        // Read all the file
        let file = fs::read_to_string("tasks.txt").expect("Unable to read file");

        println!("\n{}", file);
    } else {
        println!("\nThere's no tasks to see!");
    }

    Ok(())
}

fn remove_all() -> io::Result<()> {
    if Path::new("tasks.txt").exists() {
        fs::remove_file("tasks.txt")?;
        println!("\nAll tasks removed");
    } else {
        println!("\nThere's no tasks to remove!");
    }

    Ok(())
}

fn manage_file() -> io::Result<()> {
    if let Err(e) = create_file() {
        if e.kind() == io::ErrorKind::AlreadyExists {
            // println!("File already exists!");
            append_to_file()?;
        } else {
            return Err(e);
        }
    }
    Ok(())
}
