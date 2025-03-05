mod taskcli; // Import the taskcli module where the Command enum and parsing logic are defined
use std::io::{self, Write}; // Import necessary modules for handling input/output operations
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


        println!("{:?}\n", taskcli::Command::from_args(&args)); // Print the result of the command parsing

        

        // Optional delay to avoid flickering and allow the user to read the output
        sleep(Duration::from_millis(200));
    }
}

                // Get the first element from the 'date' slice if it exists
                // If no date is provided, fallback to the current date (formatted in DATE_FORMAT)
                // let date = date.first()
                //    .map(|&d| d.to_string())  // If a date is provided, convert it to a String
                //    .unwrap_or_else(|| Local::now().format(DATE_FORMAT).to_string()); // Default to today's date if no date is given

                // Validate if the provided date is valid (matches the expected date format)
                //if is_valid_date(&date) {
                //    return Some(Command::New(date)); // Return a new `Command::New` with the validated date
                //} else {
                //    println!("{}", INVALID_DATE); // If invalid, print an error message
                //}