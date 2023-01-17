use std::env;
use std::fs;

fn main() {
    // Get the command line arguments for the file path, specified strings and --drop flag
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let strings_to_remove: Vec<&str> = args[2..args
        .iter()
        .position(|x| x == "--drop")
        .unwrap_or(args.len())]
        .iter()
        .map(|x| x.as_str())
        .collect();
    let create_backup = !args.contains(&"--drop".to_string());

    // Read the contents of the .txt file
    let file_contents = fs::read_to_string(file_path).expect("Error reading file");

    //create a copy of the original file if --drop is not passed
    if create_backup {
        fs::copy(file_path, format!("{}.backup", file_path)).expect("Error creating backup file");
    }

    // Split the file contents into lines
    let lines: Vec<&str> = file_contents.split("\n").collect();

    // Iterate through the lines and remove those that contain the specified strings
    let new_lines: Vec<&str> = lines
        .into_iter()
        .filter(|x| !strings_to_remove.iter().any(|y| x.contains(y)))
        .collect();

    // Join the remaining lines into a single string
    let new_file_contents = new_lines.join("\n");

    // Write the new file contents to the .txt file
    fs::write(file_path, new_file_contents).expect("Error writing to file");
}
