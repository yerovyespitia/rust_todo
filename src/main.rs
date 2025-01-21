use std::io;

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    description: String,
}

fn main() {
    // Create mutable vec (array)
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        let mut action = String::new();
        println!("\nWhat do you want to do? (create, list, delete or break)");

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        let action = action.trim();

        if action == "break" {
            break;
        }

        match action.as_ref() {
            "create" => create_task(&mut tasks),
            "list" => list_tasks(&mut tasks),
            "delete" => delete_task(&mut tasks),
            _ => println!("No valid option"),
        }
    }
}

fn create_task(tasks: &mut Vec<Task>) {
    println!("\nPlease enter a title: ");

    // Declare mutable string variables
    let mut title = String::new();
    let mut description = String::new();

    // Enter input
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    println!("Please enter a description: ");

    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");

    // Create new task & add it to vec
    let task = Task {
        id: tasks.len() as u32,
        title: title.trim().to_string(),
        description: description.trim().to_string(),
    };

    // add task to vec (array)
    tasks.push(task);

    // Print new task
    println!("\nNew task added successfully");
    println!("\nTitle: {}Description: {}", title, description);
}

fn list_tasks(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("\nNo tasks available");
        return;
    }

    println!("\nTasks: ");
    for task in tasks.iter() {
        println!("{}. {} - {}", task.id + 1, task.title, task.description);
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("\nThere's no task to delete");
        return;
    }

    println!("\nInsert the task id to delete it");
    let mut id = String::new();

    io::stdin().read_line(&mut id).expect("Failed to read line");

    let number_string = id.trim();
    let id: usize = number_string.parse().expect("Not a valid number");

    let task = tasks.remove(id - 1);
    println!("Deleted: {:?}", task);
}
