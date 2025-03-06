use lists; // Import the lists module
use taskcli; // Import the taskcli module

use std::env;
use std::io::{self, Write}; // Import necessary modules for handling input/output operations
use std::path::{Path, PathBuf}; // `Path` and `PathBuf` to manage file paths
use std::thread::sleep; // Import sleep to introduce delays
use std::time::Duration; // Import Duration to specify the length of sleep

// Function to clear the terminal screen using ANSI escape codes
fn clear_screen() {
    // Clears the terminal and moves the cursor to the top-left corner
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap(); // Flush to apply changes immediately
    sleep(Duration::from_millis(100)); // Small delay to ensure the screen is cleared
}

fn main() {
    clear_screen(); // Initial screen clear before the program starts
    let mut input = String::new(); // Mutable String to store user input

    loop {
        // Clear the input buffer to prevent old input from affecting new input
        input.clear();

        // Print a prompt for the user to enter a command
        print!("Please enter a command (type 'exit' to quit): ");

        // Ensure the prompt appears before user input is taken
        io::stdout().flush().unwrap();

        // Read the user input from the console
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line"); // Expect a valid input line

        // Format the input by trimming whitespaces and converting to lowercase
        input = input.trim().to_string().to_lowercase();

        // If the user types 'exit', break the loop and end the program
        if input == "exit" {
            println!("Exiting the program...");
            break;
        }

        // Split the user input into a vector of arguments (by spaces)
        // We use Vec<&str> because the number of input arguments is dynamic
        // Vec allows for a flexible, heap-allocated collection to hold an arbitrary number of &str slices
        let args: Vec<&str> = input.split_whitespace().collect();

        // Clear the screen, then attempt to parse and execute the command based on arguments
        clear_screen();

        let program_path: Result<PathBuf, std::io::Error> = Ok(env::current_dir().unwrap());
        println!(
            "Current program path: {:#?}",
            program_path.expect("REASON").display()
        );

        match taskcli::Command::from_args(&args) {
            Some(taskcli::Command::ShowLists()) => {
                // Show lists action
                println!("Displaying the lists...");
            }
            Some(taskcli::Command::NewList(name)) => {
                // Create a new list with the given name
                println!("Creating a new list: {name}");
                match lists::List::new_list("../todo_cli/src/lists", &name) {
                    Ok(_) => println!("Successfully created the list: {name}\n"),
                    Err(e) => eprintln!("Error creating list '{name}': {e}\n"),
                }
            }
            Some(taskcli::Command::RenameList(from_name, to_name)) => {
                // Rename a list with the given name
                println!("Renaming list {from_name} to: {to_name}");
            }
            Some(taskcli::Command::DeleteList(name)) => {
                // Delete a list with the given name
                println!("Deleting list: {name}");
            }
            Some(taskcli::Command::UndefinedCommand(undefined_command)) => {
                // Undefined Command
                println!("{undefined_command}");
            }
            None => {
                continue;
            }
        }
        // Optional delay to avoid flickering and allow the user to read the output
        sleep(Duration::from_millis(200));
    }
}
