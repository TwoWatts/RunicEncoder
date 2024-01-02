
use std::fs;
mod mat;
use image::{Rgb};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please provide a single string argument representing raw binary instructions.");
        return;
    }

    if let Ok(root_dir) = std::env::current_dir() {
        println!("Root Directory: {}", root_dir.display());
    } 
    else {
        eprintln!("Failed to get the current directory.");
    }

    // Delete a directory
    let dir_to_delete = "lang"; // Specify the directory path to delete
    if let Err(err) = fs::remove_dir_all(dir_to_delete) {
        eprintln!("Failed to delete directory: {}", err);
    } 

    // Create a directory
    let dir_to_create = "lang"; // Specify the new directory path
    if let Err(err) = fs::create_dir(dir_to_create) {
        eprintln!("Failed to create directory: {}", err);
    }
    let grid: Vec<Rgb<u8>> = mat::load_image_grid("res/base16-mat.png", 18, 18, 1);
    let input = args[1].as_str().as_bytes();
    let name = args[1].as_str();
    mat::create_mat(grid, 16, 16, input, name);

}