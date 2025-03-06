use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [-p] <directory_name>...", args[0]);
        process::exit(1);
    }

    let mut create_parents = false;
    let directories = if args[1] == "-p" {
        create_parents = true;
        &args[2..]
    } else {
        &args[1..]
    };

    if directories.is_empty() {
        eprintln!("Error: No directory names provided.");
        process::exit(1);
    }

    for dir_name in directories {
        if create_parents {
            if let Err(e) = fs::create_dir_all(dir_name) {
                eprintln!("Failed to create directory '{}': {}", dir_name, e);
                process::exit(1);
            } else {
                println!("Created directory '{}' (with parents if needed)", dir_name);
            }
        } else {
            if let Err(e) = fs::create_dir(dir_name) {
                eprintln!("Failed to create directory '{}': {}", dir_name, e);
                process::exit(1);
            } else {
                println!("Created directory '{}'", dir_name);
            }
        }
    }
}