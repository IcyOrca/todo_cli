mod taskcli; // Import the taskcli module
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn clear_screen() {
    // Clears the terminal using ANSI escape codes (works in most terminals)
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap(); // Flush to apply changes immediately
    sleep(Duration::from_millis(100)); // Small delay to ensure the screen is cleared
}

fn main() {
    clear_screen(); // Initial screen clear
    let mut input = String::new();

    loop {
        // Clear the input buffer
        input.clear();

        // Print a prompt for the user
        print!("Please enter a command (type 'exit' to quit): ");

        // Read user input
        io::stdout().flush().unwrap(); // Ensure the prompt appears before input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Formats the input
        input = input.trim().to_string().to_lowercase();

        // If the user types 'exit', break the loop and exit
        if input == "exit" {
            println!("Exiting the program...");
            break;
        }

        // Split the input string into individual arguments
        let args: Vec<&str> = input.split_whitespace().collect();

        // Attempt to parse the command from the arguments
        clear_screen();
        println!("{:?}\n", taskcli::Command::from_args(&args));

        // Optional delay to avoid flickering
        sleep(Duration::from_millis(200));
    }
}
