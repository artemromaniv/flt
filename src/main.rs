use std::env;
use std::fs;

fn main() {
    // Get the command line arguments for the file path, specified strings and flags
    let args: Vec<String> = env::args().collect();

    // Check for --help flag
    if args.contains(&"--help".to_string()) {
        println!(
            "Usage: flt [file_path] <strings_to_remove>\n
Flags:\n
--drop\t\tdisables creating backup of file before filtering\n 
--nompt\t\tremoves empty lines from your file"
        );

        return;
    }

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error: {:?}", err);
        std::process::exit(1);
    });

    // println!("ARGS: {:#?}",args);
    let file_path = config.file_path;

    let strings_to_remove = config.query;

    // Flags

    let create_backup = !args.contains(&"--drop".to_string());

    let remove_empty_lines = args.contains(&"--nompt".to_string());

    // Read the contents of the .txt file
    let file_contents = fs::read_to_string(&file_path).expect("Error reading file");

    //create a copy of the original file if --drop is not passed
    if create_backup {
        fs::copy(&file_path, format!("{}.backup", &file_path)).expect("Error creating backup file");
    }

    // Split the file contents into lines
    let lines: Vec<&str> = file_contents.split("\n").collect();

    // Iterate through the lines and remove those that contain the specified strings
    let new_lines: Vec<&str> = lines
        .into_iter()
        .filter(|x| !strings_to_remove.iter().any(|y| x.contains(y)))
        .filter(|x| {
            !strings_to_remove.iter().any(|y| x.contains(y))
                && (!remove_empty_lines || x.trim() != "")
        })
        .collect();

    // Join the remaining lines into a single string
    let new_file_contents = new_lines.join("\n");

    // Write the new file contents to the .txt file
    fs::write(file_path, new_file_contents).expect("Error writing to file");
}

struct Config {
    file_path: String,
    query: Vec<String>,
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            println!("No arguments were provided");
            println!("Use --help flag for usage instructions");
            return Err("Not enough arguments.");
        }

        let file_path = args[1].to_string();

        let query: Vec<String> = args[2..args
            .iter()
            .position(|x| x == "--drop" || x == "--nompt")
            .unwrap_or(args.len())]
            .iter()
            .map(|x| x.to_string())
            .collect();

        Ok(Config { file_path, query })
    }
}
