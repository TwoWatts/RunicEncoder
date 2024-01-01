use image::{DynamicImage, GenericImageView, Rgb};
use image::{ImageBuffer};
use std::fs;

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
                    let pixel = rgb.get_pixel(xx, yy).0;
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

fn create_mat(image_grid: Vec<Rgb<u8>>, char_width: u32, char_height: u32, eles: [u8; 9], name: &str) {

    // Split image_grid into chunks representing individual images
    let images: Vec<&[Rgb<u8>]> = image_grid.chunks((char_width * char_height) as usize).collect();

    // The Mat
    let mat: Vec<Rgb<u8>> = Vec::with_capacity(eles.len() * (char_width*char_height) as usize);

    // Output Image
    let mat_pixel_width = char_width * 3;
    let mat_pixel_height = char_height * 3;
    let mut img_buffer = ImageBuffer::new(mat_pixel_width, mat_pixel_height);

    // Fill Out Mat Patterns
    let mut p1 = 0;
    let mut p2 = 0;
    for (idx, ele) in eles.iter().enumerate() {
        let image_chunk = images[*ele as usize];
        for (i, pixel) in image_chunk.iter().enumerate() {
            let x = (i % char_height as usize) as u32;
            let y = (i / char_width as usize) as u32;
            img_buffer.put_pixel(x+p1*char_width, y+p2*char_height, *pixel);
        }

        if ( p1 == 2 ) {
            p1 = 0;
            p2 += 1;
        }
        else {
            p1 += 1;
        }
    }

    // Save the ImageBuffer as an image file (e.g., PNG)
    let file_path = format!("{}.png", name);
    img_buffer.save(file_path).expect("Failed to save image chunk");
}

fn encoder(input: &str) -> [u8; 9] {
    return [8, 16, 22, 34, 58, 100, 107, 112, 116];
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
    let grid: Vec<Rgb<u8>> = load_image_grid("base16-mat.png", 18, 18, 1);
    let input = encoder("test");
    create_mat(grid, 16, 16, input, "test");

}