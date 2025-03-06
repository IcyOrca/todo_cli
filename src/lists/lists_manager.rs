// Import necessary modules for file handling and error management
use std::fs::File;
use std::io::{self, Write}; // `io` for handling errors, `Write` for potential future file writing
use std::path::{Path, PathBuf}; // `Path` and `PathBuf` to manage file paths

// Define a constant for the file extension used for lists
const TXT_EXTENSION: &str = ".txt";

// Define a struct `List` that represents a task list, where each list is stored as a file
#[derive(Debug)] // Allows the struct to be printed for debugging purposes
pub struct List {
    path: PathBuf, // Stores the path to the list file
}

impl List {
    // Function to create a new list file in the specified directory
    pub fn new_list(directory: &str, name: &str) -> io::Result<Self> {
        // Construct the full file path for the new list
        let file_path = Path::new(directory).join(format!("{name}{TXT_EXTENSION}"));

        // Check if the file already exists to avoid overwriting an existing list
        if file_path.exists() {
            let e = format!("The List {name} already exists."); // Error message
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, e)); // Return an error
        }

        // Attempt to create a new file
        match File::create(&file_path) {
            Ok(_) => {
                Ok(List { path: file_path }) // Return a `List` struct with the created file's path
            }
            Err(e) => {
                Err(e) // Propagate the error to the caller
            }
        }
    }
}
