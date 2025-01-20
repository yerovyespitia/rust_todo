use std::io;

fn main() {
    loop {
        let mut action = String::new();
        println!("What do you want to do?");

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        if action.trim() == "break" {
            break;
        }

        match action.as_ref() {
            "create" => create_task(),
            _ => println!("No valid option"),
        }
    }
}

fn create_task() {
    println!("Please enter a title: ");

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

    // Print new task
    println!("\nNew task added");
    println!("\nTitle: {}Description: {}", title, description);
}
