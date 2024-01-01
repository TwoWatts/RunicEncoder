use image::{DynamicImage, GenericImageView, Rgb};
use image::{ImageBuffer};
use std::fs;
use rand::Rng;

const CRC8_POLY: u8 = 0x07; // CRC-8 polynomial
const CRC8_TABLE: [u8; 256] = [
    0x00, 0x07, 0x0E, 0x09, 0x1C, 0x1B, 0x12, 0x15,
    0x38, 0x3F, 0x36, 0x31, 0x24, 0x23, 0x2A, 0x2D,
    0x70, 0x77, 0x7E, 0x79, 0x6C, 0x6B, 0x62, 0x65,
    0x48, 0x4F, 0x46, 0x41, 0x54, 0x53, 0x5A, 0x5D,
    0xE0, 0xE7, 0xEE, 0xE9, 0xFC, 0xFB, 0xF2, 0xF5,
    0xD8, 0xDF, 0xD6, 0xD1, 0xC4, 0xC3, 0xCA, 0xCD,
    0x90, 0x97, 0x9E, 0x99, 0x8C, 0x8B, 0x82, 0x85,
    0xA8, 0xAF, 0xA6, 0xA1, 0xB4, 0xB3, 0xBA, 0xBD,
    0xC7, 0xC0, 0xC9, 0xCE, 0xDB, 0xDC, 0xD5, 0xD2,
    0xFF, 0xF8, 0xF1, 0xF6, 0xE3, 0xE4, 0xED, 0xEA,
    0xB7, 0xB0, 0xB9, 0xBE, 0xAB, 0xAC, 0xA5, 0xA2,
    0x8F, 0x88, 0x81, 0x86, 0x93, 0x94, 0x9D, 0x9A,
    0x27, 0x20, 0x29, 0x2E, 0x3B, 0x3C, 0x35, 0x32,
    0x1F, 0x18, 0x11, 0x16, 0x03, 0x04, 0x0D, 0x0A,
    0x57, 0x50, 0x59, 0x5E, 0x4B, 0x4C, 0x45, 0x42,
    0x6F, 0x68, 0x61, 0x66, 0x73, 0x74, 0x7D, 0x7A,
    0x89, 0x8E, 0x87, 0x80, 0x95, 0x92, 0x9B, 0x9C,
    0xB1, 0xB6, 0xBF, 0xB8, 0xAD, 0xAA, 0xA3, 0xA4,
    0xF9, 0xFE, 0xF7, 0xF0, 0xE5, 0xE2, 0xEB, 0xEC,
    0xC1, 0xC6, 0xCF, 0xC8, 0xDD, 0xDA, 0xD3, 0xD4,
    0x69, 0x6E, 0x67, 0x60, 0x75, 0x72, 0x7B, 0x7C,
    0x51, 0x56, 0x5F, 0x58, 0x4D, 0x4A, 0x43, 0x44,
    0x19, 0x1E, 0x17, 0x10, 0x05, 0x02, 0x0B, 0x0C,
    0x21, 0x26, 0x2F, 0x28, 0x3D, 0x3A, 0x33, 0x34,
    0x4E, 0x49, 0x40, 0x47, 0x52, 0x55, 0x5C, 0x5B,
    0x76, 0x71, 0x78, 0x7F, 0x6A, 0x6D, 0x64, 0x63,
    0x3E, 0x39, 0x30, 0x37, 0x22, 0x25, 0x2C, 0x2B,
    0x06, 0x01, 0x08, 0x0F, 0x1A, 0x1D, 0x14, 0x13,
    0xAE, 0xA9, 0xA0, 0xA7, 0xB2, 0xB5, 0xBC, 0xBB,
    0x96, 0x91, 0x98, 0x9F, 0x8A, 0x8D, 0x84, 0x83,
    0xDE, 0xD9, 0xD0, 0xD7, 0xC2, 0xC5, 0xCC, 0xCB,
    0xE6, 0xE1, 0xE8, 0xEF, 0xFA, 0xFD, 0xF4, 0xF3,
];

fn crc8(data: &[u8]) -> u8 {
    let mut crc = 0u8;
    for &byte in data {
        crc = CRC8_TABLE[(crc ^ byte) as usize];
    }
    crc
}

fn load_image_grid(path: &str, rect_width: u32, rect_height: u32, border_width: u32) -> Vec<Rgb<u8>> {
    // Open Image File
    let err_fp_dne = format!("Failed to open image file: {}", path);
    let img = image::open(path).expect(err_fp_dne.as_str());

    // Convert Image To RGB
    let rgb: ImageBuffer<Rgb<u8>, Vec<u8>> = img.into_rgb8();

    // Derive Grid Parsing Parameters
    let (width, height) = rgb.dimensions();
    let start_x = border_width;
    let start_y = border_width;
    let end_x = width-1-border_width;
    let end_y = height-1-border_width;
    let segs_per_row = (end_x-start_x)  / (rect_width);
    let segs_per_col = (end_y-start_y) / (rect_height);
    let n_rectangles = segs_per_row * segs_per_col;
    let n_pixels = n_rectangles * rect_width * rect_height;
    let char_width = rect_width - 2 * border_width;
    let char_height = rect_height - 2 * border_width;

    println!("Image Width = {}\nImage Height = {}\nPixels = {}\nRectangles = {}", width, height, n_pixels, n_rectangles);

    // Allocate Memory For Image Grid
    let mut image_grid: Vec<Rgb<u8>> = Vec::with_capacity(n_pixels as usize);

    // Add Pixels To Image Grid
    for y in (start_y..end_y).step_by((rect_height-border_width) as usize) {
        for x in (start_x..end_x).step_by((rect_width-border_width) as usize) {
            for yy in y..(y + char_height) {
                if yy > end_y {
                    break;
                }
                for xx in x..(x + char_width) {
                    if xx > end_x {
                        break;
                    }
                    let mut pixel = rgb.get_pixel(xx, yy).0;
                    image_grid.push(Rgb(pixel));
                }
            }
        }
    }

    image_grid

}

fn read_image_grid(image_grid: Vec<Rgb<u8>>, char_width: u32, char_height: u32) {
    // Split image_grid into chunks representing individual images
    let images: Vec<&[Rgb<u8>]> = image_grid.chunks((char_width * char_height) as usize).collect();
    //println!("Output Grid Length = {}", images.len());

    // // Write each image chunk to be its own separate file.
    for (index, image_chunk) in images.iter().enumerate() {
        // Create an ImageBuffer from the Rgb<u8> slice
        let mut img_buffer = ImageBuffer::new(char_width, char_height);
        for (i, pixel) in image_chunk.iter().enumerate() {
            let x = (i % char_height as usize) as u32;
            let y = (i / char_width as usize) as u32;
            img_buffer.put_pixel(x, y, *pixel);
        }

        // Save the ImageBuffer as an image file (e.g., PNG)
        let hex_digit = format!("{:x}", index);
        let file_path = format!("lang/number_{}.png", hex_digit);
        img_buffer.save(file_path).expect("Failed to save image chunk");
    }
}

fn create_mat(image_grid: Vec<Rgb<u8>>, char_width: u32, char_height: u32, eles: &[u8], name: &str) {

    // Split image_grid into chunks representing individual images
    let images: Vec<&[Rgb<u8>]> = image_grid.chunks((char_width * char_height) as usize).collect();

    // The Mat
    let mat: Vec<Rgb<u8>> = Vec::with_capacity(eles.len() * (char_width*char_height) as usize);
    let mat_dim = (((eles.len()) as f32).sqrt().ceil()) as u32;
    println!("Dimension = {}", mat_dim);
    let mat_pixel_width = char_width * mat_dim;
    let mat_pixel_height = char_height * mat_dim;

    // Color
    let mut rng = rand::thread_rng(); // Initialize the random number generator
    let rand1: u8 = rng.gen(); // Generate a random u8
    let rand2: u8 = rng.gen(); // Generate a random u8
    let rand3: u8 = rng.gen(); // Generate a random u8
    let mut color1 = 0;
    let mut color2 = 0;
    let mut color3 = 0;

    // Output Image
    let mut img_buffer = ImageBuffer::new(mat_pixel_width, mat_pixel_height);

    // Fill Out Mat Patterns
    let mut p1 = 0;
    let mut p2 = 0;
    print!("Input: ");
    for (idx, &ele) in eles.iter().enumerate() {
        print!("{} ", ele);
        let image_chunk = images[ele as usize];
        for (i, &pixel) in image_chunk.iter().enumerate() {
            let x = (i % char_height as usize) as u32;
            let y = (i / char_width as usize) as u32;
            let mut rgb_val = pixel.0;

            if (rgb_val[0] != 0xFF) {
                rgb_val[0] = color1;
            } else {
                rgb_val[0] = !color1;
            }
            if (rgb_val[1] != 0xFF) {
                rgb_val[1] = color2;
            } else {
                rgb_val[1] = !color2;
            }
            if (rgb_val[2] != 0xFF) {
                rgb_val[2] = color3;
            } else {
                rgb_val[2] = !color3;
            }
            img_buffer.put_pixel(x+p1*char_width, y+p2*char_height, Rgb(rgb_val));
        }

        if ( p1 == (mat_dim-1) ) {
            p1 = 0;
            p2 += 1;
        }
        else {
            p1 += 1;
        }
    }
    print!("\n");
    
    // Save the ImageBuffer as an image file (e.g., PNG)
    let file_path = format!("{}.png", name);
    img_buffer.save(file_path).expect("Failed to save image chunk");
}

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
    let grid: Vec<Rgb<u8>> = load_image_grid("res/base16-mat.png", 18, 18, 1);
    let input = args[1].as_str().as_bytes();
    let name = args[1].as_str();
    create_mat(grid, 16, 16, input, name);

}