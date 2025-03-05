// Importing Local (for local time) and NaiveDate (for handling dates without time or timezone) from the chrono crate
use chrono::{Local, NaiveDate};

// Create consts as string literals for the commands and messages to be used in the parser function
const TASKCLI: &str = "taskcli";
const HELP: &str = "help";
const NEW: &str = "new";
const LIST: &str = "list";
const ADD: &str = "add";
const UPDATE: &str = "update";
const DELETE: &str = "delete";
const DATE_FORMAT: &str = "%Y-%m-%d";
const INVALID_DATE: &str = "Invalid date format. Please use YYYY-MM-DD.";
const INVALID_COMMAND: &str = "Invalid command! Please use {TASKCLI} {HELP} to check the available commands.";

// Create an enum to represent the commands that the user can input in the CLI application
// Derive the Debug trait to allow the enum to be printed for debugging purposes 
#[derive(Debug)]
pub enum Command {
    List(String),
    Add(String, String),
    Update(String, usize, String),
    Delete(String, usize),
    New(String),
}

// Implement a function to parse the command line arguments and return a Command enum instance
impl Command {

    // The from_args function takes a slice of string arguments and returns an Option<Command> instance
    pub fn from_args(args: &[&str]) -> Option<Command> {

        // Helper function to validate date format using the NaiveDate struct from the chrono crate
        fn is_valid_date(date: &str) -> bool {
            // Check if the date string can be parsed into a NaiveDate instance using the DATE_FORMAT constant as the format string 
            NaiveDate::parse_from_str(date, DATE_FORMAT).is_ok()
        }
        
        // Match the arguments to check for the command and its arguments
        match args {

            // Check if the arguments are equal to the help command and return the help message with the available commands
            [TASKCLI, cmd] if *cmd == HELP => {
                println!("Available commands:");
                println!("{TASKCLI} {HELP} - Show this help message");
                println!("{TASKCLI} {NEW} <date> - Create a new task file for a specific date");
                println!("{TASKCLI} {LIST} <date> - List tasks from a specific file");
                println!("{TASKCLI} {ADD} <date> <task> - Add a task to a specific date's file");
                println!("{TASKCLI} {UPDATE} <date> <id> <task> - Update a task in a specific date's file");
                println!("{TASKCLI} {DELETE} <date> <id> - Delete a task from a specific date's file");
            }
            
            // Check if the arguments are equal to the new command and return a new Command instance with the date argument
            // "@ .." Binds the remaining elements (after "new") to the variable `date` as a slice of string slices (&str)
            [TASKCLI, cmd, date @ ..] if cmd == NEW => {
                // Get the first element from the 'date' slice if it exists
                // If no date is provided, fallback to the current date (formatted in DATE_FORMAT)
                let date = date.first()
                    .map(|&d| d.to_string())  // If a date is provided, convert it to a String
                    .unwrap_or_else(|| Local::now().format(DATE_FORMAT).to_string()); // Default to today's date if no date is given

                // Validate if the provided date is valid (matches the expected date format)
                if is_valid_date(&date) {
                    return Some(Command::New(date)); // Return a new `Command::New` with the validated date
                } else {
                    println!("{}", INVALID_DATE); // If invalid, print an error message
                }
            }

            // Default case for invalid command
            _ => {
                println!("{INVALID_COMMAND}");
            }
        }
        
        // Return None if no valid command was matched
        return None;

    }
}
