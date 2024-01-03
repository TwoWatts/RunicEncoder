mod mat;
use std::{fs};
use std::io::{self, Write};
use image::{Rgb};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Ok(root_dir) = std::env::current_dir() {
        println!("Root Directory: {}", root_dir.display());
    } 
    else {
        eprintln!("Failed to get the current directory.");
    }

    // // Delete a directory
    // let dir_to_delete = "lang"; // Specify the directory path to delete
    // if let Err(err) = fs::remove_dir_all(dir_to_delete) {
    //     eprintln!("Failed to delete directory: {}", err);
    // } 

    // // Create a directory
    // let dir_to_create = "lang"; // Specify the new directory path
    // if let Err(err) = fs::create_dir(dir_to_create) {
    //     eprintln!("Failed to create directory: {}", err);
    // }

    // Menu Prompt
    println!("Select an option:");
    println!("[0] Encode rune.");
    println!("[1] Decode rune.");
    println!("[2] Quit.");
    print!(">> ");
    io::stdout().flush().unwrap();

    // Accept Input
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("ERROR: Could not read input.");
    let answer: u32= option.trim().parse().unwrap();
    
    if answer == 0 {
        println!("Encode your rune by entering text and pressing <ENTER>...");
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("ERROR: Could not read input.");
        let text = input.as_str().trim();
        let text_bytes = text.as_bytes();
        //println!("You entered: {:?}", text);
        println!("Rune Succeeded! See \"{}.png\"...", text);
        let grid: Vec<Rgb<u8>> = mat::load_image_grid("res/base16-mat.png", 18, 18, 1);
        mat::create_mat(grid, 16, 16, text_bytes, text);
    }


}