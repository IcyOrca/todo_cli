// Importing Local (for local time) and NaiveDate (for handling dates without time or timezone) from the chrono crate
use chrono::{Local, NaiveDate};

// Create consts as string literals for the commands and messages to be used in the parser function
const TASKCLI: &str = "taskcli";
const LIST: &str = "list";
const LISTS: &str = "list";
const TASK: &str = "task";
const HELP: &str = "help";
const SHOW: &str = "show";
const NEW: &str = "new";
const RENAME: &str = "rename";
const UPDATE: &str = "update";
const DELETE: &str = "delete";

const DATE_FORMAT: &str = "%Y-%m-%d";
const INVALID_DATE: &str = "Invalid date format. Please use YYYY-MM-DD.";
const UNDEFINED_COMMAND: &str =
    "Undefined Command! Please use {TASKCLI} {HELP} to check the available commands.";

// Create an enum to represent the commands that the user can input in the CLI application
// Derive the Debug trait to allow the enum to be printed for debugging purposes
#[derive(Debug)]
pub enum Command {
    ShowLists(),
    NewList(String),
    RenameList(String, String),
    DeleteList(String),
    UndefinedCommand(String),
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
            // [TASKCLI, cmd] if *cmd == HELP => {
            // @TODO LATER
            //  }

            // Return a new `Command::ShowLists` with the list's name
            [TASKCLI, SHOW, LISTS] => {
                return Some(Command::ShowLists());
            }

            // Return a new `Command::NewList` with the list's name
            [TASKCLI, NEW, LIST, name] => {
                return Some(Command::NewList(String::from(name.trim())));
            }

            // Return a new `Command::RenameList` with the list's name
            [TASKCLI, RENAME, LIST, from_name, to_name] => {
                return Some(Command::RenameList(
                    String::from(from_name.trim()),
                    String::from(to_name.trim()),
                ));
            }

            // Return a new `Command::DeleteList` with the list's name
            [TASKCLI, DELETE, LIST, name] => {
                return Some(Command::DeleteList(String::from(name.trim())));
            }

            // Default case for invalid command
            _ => {
                return Some(Command::UndefinedCommand(String::from(UNDEFINED_COMMAND)));
            }
        }
    }
}
