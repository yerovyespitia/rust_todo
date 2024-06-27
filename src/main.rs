use std::io;

fn main() {
    println!("Please enter a title: ");

    let mut title = String::new();
    let mut description = String::new();

    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    println!("Please enter a description: ");

    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");

    println!("Title: {title}");
    println!("Description: {description}");
}
