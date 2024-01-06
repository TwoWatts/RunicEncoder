use std::{io::{self, Write}};

pub fn process_digit_input() -> Result<i64, i8> {
    // Flush Output Buffer
    print!(">> ");
    io::stdout().flush().unwrap();

    // Accept Input
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {println!("Failed to read line."); return Err(-1)}
    }

    // Set Text
    let text = input.trim();
    if text.is_empty() {
        println!("No input provided."); 
        return Err(-2);
    }

    // Parse Digit
    match text.parse::<i64>() {
        Ok(num) => Ok(num),
        Err(_) => {
            Err(-3)
        }
    }
}

pub fn process_string_input() -> Result<String, i8> {
    // Flush Output Buffer
    print!(">> ");
    io::stdout().flush().unwrap();

    // Accept Input
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {println!("Failed to read line."); return Err(-1)}
    }

    // Set Text
    let text = input.trim();
    if text.is_empty() {
        println!("No input provided."); 
        return Err(-2);
    }

    return Ok(text.to_string());
}

pub fn accept_digit_input() -> i64 {
    let mut ret: Result<i64, i8>;
    let mut keep_going: bool = true;

    while keep_going == true {
        ret = process_digit_input();
        match ret {
            Ok(value) => {
                keep_going = false;
                return value;
            }
            Err(error) => {
                println!("Expected digit, but received something else.");
                keep_going = true;
            }
        }
    }
    return 0;
}

pub fn accept_string_input() -> String {
    let mut ret: Result<String, i8>;
    let mut keep_going: bool = true;

    while keep_going == true {
        ret = process_string_input();
        match ret {
            Ok(value) => {
                keep_going = false;
                return value;
            }
            Err(error) => {
                println!("Expected string, but received something else.");
                keep_going = true;
            }
        }
    }
    return "".to_string();
}

pub fn collect_wd_runes() -> Vec<String> {
    let mut rune_paths: Vec<String> = Vec::new();
    let mut rune_count = 0;
    if let Ok(entries) = std::fs::read_dir(".") {
        for entry_result in entries {
            match entry_result {
                Ok(entry) => {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            // Do nothing.
                        } else {
                            let mut name = entry.file_name().into_string().unwrap();
                            if name.contains(".png") {
                                println!("Rune {}: {:?}", rune_count, entry.file_name());
                                rune_paths.push(name);
                                rune_count += 1;
                            }
                        }
                    } else {
                        println!("Failed to get file type for entry");
                    }
                }
                Err(err) => {
                    println!("Error reading directory entry: {}", err);
                }
            }
        }
    } else {
        println!("Failed to read directory");
    }
    rune_paths
}