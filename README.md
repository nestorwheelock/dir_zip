
# dir_zip

**dir_zip** is a Rust-based tool that creates password-protected ZIP files from directories. It retrieves the password either from an environment variable or prompts the user to input one if not set. It also supports parallel file processing for faster performance.

## Features

- Creates password-protected ZIP files.
- Retrieves the password from the `ZIP_PASSWORD` environment variable.
- Prompts the user to enter the password if the environment variable is not set.
- Supports parallel file processing using the `rayon` crate for faster processing.

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/dir_zip.git
   cd dir_zip
   ```

2. **Build the project using Cargo**:
   ```bash
   cargo build --release
   ```

## Usage

### Setting the Password:

1. **Using Environment Variable**:
   The program can use the `ZIP_PASSWORD` environment variable to securely set the password. This is the recommended method as it keeps the password outside of your code.

   - **On Linux/macOS**:
     ```bash
     export ZIP_PASSWORD="your_password"
     ```

   - **On Windows**:
     ```bash
     set ZIP_PASSWORD="your_password"
     ```

   After setting the environment variable, you can run the program, and it will use the password from the variable to create the ZIP files.

2. **Prompting for Password**:
   If the `ZIP_PASSWORD` environment variable is not set, the program will prompt you to enter a password at runtime.

### Running `dir_zip`:

To zip the files in a directory, run the program and provide the directory path as an argument:

```bash
./target/release/dir_zip /path/to/directory
```

### Example 1: Using Environment Variable

1. **Set the environment variable**:
   ```bash
   export ZIP_PASSWORD="supersecret123"
   ```

2. **Run the program**:
   ```bash
   ./target/release/dir_zip /path/to/directory
   ```

3. **Expected Output**:
   ```
   Successfully created password-protected ZIP: file1.zip
   Successfully created password-protected ZIP: file2.zip
   Successfully created password-protected ZIP: file3.zip
   ```

### Example 2: Prompting for Password

1. **Run the program without setting the environment variable**:
   ```bash
   ./target/release/dir_zip /path/to/directory
   ```

2. **Program will prompt for a password**:
   ```
   The `ZIP_PASSWORD` environment variable is not set.
   You can set it by running:
   For Linux/macOS: export ZIP_PASSWORD="your_password"
   For Windows: set ZIP_PASSWORD="your_password"

   Please enter a password for the ZIP file:
   ```

3. **Enter the password**:
   ```
   mypassword123
   ```

4. **Expected Output**:
   ```
   Successfully created password-protected ZIP: file1.zip
   Successfully created password-protected ZIP: file2.zip
   Successfully created password-protected ZIP: file3.zip
   ```

### Example 3: Handling Errors

1. **Invalid directory provided**:
   If you run the program with an invalid directory, the program will output an error:
   ```bash
   ./target/release/dir_zip /invalid/path
   ```

2. **Expected Output**:
   ```
   The provided path is not a valid directory.
   ```

### Detailed Explanation:

- The program zips each file or folder inside the provided directory into a separate password-protected ZIP file.
- It uses the `rayon` crate for parallel processing, meaning the files will be zipped in parallel for better performance on large directories.
- The password can be provided either through an environment variable (`ZIP_PASSWORD`) or via a prompt if the variable is not set.
- The ZIP files will be created in the same directory as the files that are being zipped.

### Additional Notes:

- If the `ZIP_PASSWORD` environment variable is not set, the program provides instructions for setting it.
- The program will prompt for a password only if the environment variable is missing.
- The ZIP files will be created using the `zip` command-line tool, so ensure that `zip` is installed on your system.

### License

This project is licensed under the GNU GPLv3 License. See the [LICENSE](LICENSE) file for more details.
