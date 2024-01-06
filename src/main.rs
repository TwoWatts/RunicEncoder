mod mat;
mod cli;
use std::io::{self, Write};
use image::Rgb;

use crate::cli::{accept_digit_input, accept_string_input};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Ok(root_dir) = std::env::current_dir() {
        println!("Root Directory: {}", root_dir.display());
    } 
    else {
        eprintln!("Failed to get the current directory.");
    }

    // Menu Prompt
    println!("Select an option:");
    println!("[0] Encode rune.");
    println!("[1] Decode rune.");
    println!("[2] Quit.");

    // Accept User Input
    let option = accept_digit_input();
    
    if option == 0 {
        // Ask User For Rune Seed
        println!("Encode your rune by entering text and pressing <ENTER>...");
        io::stdout().flush().unwrap();

        // Accept User Input Seed
        let text = accept_string_input();
        let text_bytes = text.as_bytes();

        //println!("You entered: {:?}", text_bytes);
        println!("Rune Succeeded! See \"{}.png\"", text);
        let grid: Vec<Rgb<u8>> = mat::load_image_grid("res/base16-mat.png", 18, 18, 1);
        mat::create_mat(grid, 16, 16, text_bytes, &text);
    }
    else if option == 1 {
        // Ask User For Rune Name
        println!("Which rune should be decoded?");
        let avail_runes = cli::collect_wd_runes();
        let rune_idx: usize = accept_digit_input() as usize;

        if rune_idx > 0 && rune_idx < avail_runes.len() { 
            let rune = &avail_runes[rune_idx];
        }

    }
    else if option == 2 {
        return;
    }
    else {
        println!("Unknown option selected. Exiting program.");
    }

}