use rayon::prelude::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::io::{self, Write};

/// Reads the password from the `ZIP_PASSWORD` environment variable.
/// 
/// Returns:
/// - `Some(String)` if the environment variable is set.
/// - `None` if the environment variable is not set.
fn get_password_from_env() -> Option<String> {
    env::var("ZIP_PASSWORD").ok()
}

/// Prompts the user to enter a password for the ZIP file.
/// 
/// Returns:
/// - A `Result` containing the password as a `String` if successful.
/// - An `io::Error` if the input could not be read.
fn prompt_for_password() -> io::Result<String> {
    let mut password = String::new();
    print!("Please enter a password for the ZIP file: ");
    io::stdout().flush()?;  // Ensure the prompt is displayed before input
    io::stdin().read_line(&mut password)?;
    Ok(password.trim().to_string())  // Remove any trailing newlines or spaces
}

/// Zips the specified input files into a password-protected ZIP archive.
/// 
/// Arguments:
/// - `output_zip`: The name of the output ZIP file.
/// - `input_files`: A list of input files to be zipped.
/// - `password`: The password to protect the ZIP file.
/// 
/// Returns:
/// - `Ok(())` if the ZIP was created successfully.
/// - An `io::Error` if the command failed to execute.
fn zip_with_password(output_zip: &str, input_files: &[&str], password: &str) -> io::Result<()> {
    let mut command = Command::new("zip");
    command.arg("-erP").arg(password).arg(output_zip);
    
    // Add all input files to the zip command
    for input in input_files {
        command.arg(input);
    }

    // Execute the zip command
    let status = command.status()?;

    if status.success() {
        println!("Successfully created password-protected ZIP: {}", output_zip);
    } else {
        eprintln!("Error creating ZIP file.");
    }

    Ok(())
}

/// Main entry point for the program.
/// 
/// - Reads the directory to zip from the command-line arguments.
/// - Attempts to retrieve the password from the environment variable `ZIP_PASSWORD`.
/// - Prompts for a password if the environment variable is not set.
/// - Provides instructions on how to export the password as an environment variable.
/// - Uses parallel processing with Rayon to zip files.
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <directory_with_files_and_folders_to_zip>", args[0]);
        return;
    }

    let main_directory = PathBuf::from(&args[1]);

    // Check if the provided directory exists
    if !main_directory.exists() || !main_directory.is_dir() {
        eprintln!("The provided path is not a valid directory.");
        return;
    }

    // Get the password from the environment variable or prompt for one
    let password = if let Some(env_password) = get_password_from_env() {
        env_password
    } else {
        // Remind the user to export the password as an environment variable
        eprintln!(
            "The `ZIP_PASSWORD` environment variable is not set.\n\
            You can set it by running:\n\
            For Linux/macOS: export ZIP_PASSWORD=\"your_password\"\n\
            For Windows: set ZIP_PASSWORD=\"your_password\"\n"
        );

        // Prompt for the password if the environment variable is not set
        match prompt_for_password() {
            Ok(password) => password,
            Err(e) => {
                eprintln!("Failed to read password: {}", e);
                return;
            }
        }
    };

    // Get all files and directories inside the main directory
    let entries: Vec<PathBuf> = fs::read_dir(&main_directory)
        .expect("Unable to read the directory")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();

    // Use rayon for parallel processing of each file/directory
    entries.par_iter().for_each(|entry| {
        if let Some(file_stem) = entry.file_stem() {
            if let Some(file_name_str) = file_stem.to_str() {
                let output_zip = format!("{}.zip", file_name_str); // Name ZIP file after the directory/file
                
                if let Err(e) = zip_with_password(&output_zip, &[entry.to_str().unwrap()], &password) {
                    eprintln!("Error zipping file {}: {}", entry.display(), e);
                }
            }
        }
    });
}

