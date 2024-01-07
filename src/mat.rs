use rand::Rng;
use crypto_hash::{Algorithm, hex_digest};
use image::{DynamicImage, GenericImageView, Rgb};
use image::{ImageBuffer};
use std::fs;

fn pick_color(data: &[u8]) -> (u8, u8, u8) {
    if data.is_empty() {
        return (0, 0, 0);
    }

    // Calculate a hash from the input data
    let hash = hex_digest(Algorithm::SHA256, data);

    // Take three pairs of characters from the hash string and convert them to u8 values
    let color1 = u8::from_str_radix(&hash[0..2], 16).unwrap_or(0);
    let color2 = u8::from_str_radix(&hash[2..4], 16).unwrap_or(0);
    let color3 = u8::from_str_radix(&hash[4..6], 16).unwrap_or(0);

    (color1, color2, color3)
}

pub fn load_image_grid(path: &str, rect_width: u32, rect_height: u32, border_width: u32) -> Vec<Rgb<u8>> {
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

    //println!("Image Width = {}\nImage Height = {}\nPixels = {}\nRectangles = {}", width, height, n_pixels, n_rectangles);

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

pub fn read_image_grid(image_grid: Vec<Rgb<u8>>, char_width: u32, char_height: u32) {
    // Split image_grid into chunks representing individual images
    let images: Vec<&[Rgb<u8>]> = image_grid.chunks((char_width * char_height) as usize).collect();
    //println!("Output Grid Length = {}", images.len());

    // Write each image chunk to be its own separate file.
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

fn spiral_walker(dim: u32) -> Vec<(u32,u32)> {
    let tot_steps = dim * dim;
    let mut step_count = 0;
    let mut path: Vec<(u32, u32)>= Vec::with_capacity(tot_steps as usize);

    // Boundaries
    let mut top = 0;
    let mut bottom = dim-1;
    let mut left = 0;
    let mut right = dim-1;

    loop {

        for i in (left..=right).rev() {
            path.push((i, top));
            step_count+=1;
        }
        for i in (top+1)..=bottom {
            path.push((left, i));
            step_count+=1;
        }
        for i in (left+1)..=right {
            path.push((i, bottom));
            step_count+=1;
        }
        for i in (top+1..bottom).rev() {
            path.push((right, i));
            step_count+=1;
        }

        if step_count >= tot_steps {
            break;
        }

        bottom-=1;
        top+=1;
        left+=1;
        right-=1;
    }

    return path;
}

pub struct Mat {
    font: Vec<Rgb<u8>>,
    char_width: u32,
    char_height: u32,
}

impl Mat {

    pub fn new(char_width: u32, char_height: u32, border_width: u32) -> Self {
        // Parse font characters as grid.
        let grid: Vec<Rgb<u8>> = load_image_grid("res/base16-mat.png", char_width + 2 * border_width, char_height + 2 * border_width, border_width);

        let mat = Mat {
            font: grid,
            char_width,
            char_height,
        };

        mat
    }

    pub fn export(&self, input: &[u8], name: &str) {
        // The Mat
        let mat_dim = (((input.len()) as f32).sqrt().ceil()) as u32;
        let mat_pixel_width = self.char_width * mat_dim;
        let mat_pixel_height = self.char_height * mat_dim;
        let images: Vec<&[Rgb<u8>]> = self.font.chunks((self.char_width * self.char_height) as usize).collect();
        let imperfection = (mat_dim*mat_dim) as usize - input.len();
        let mut ideal_input = input.to_vec();
        if imperfection > 0 {
            for i in 0..imperfection {
                ideal_input.push(' ' as u8);
            }
        }
    
        // Color
        let (color1, color2, color3) = pick_color(input);
    
        // Output Image
        let mut img_buffer = ImageBuffer::new(mat_pixel_width as u32, mat_pixel_height as u32);
    
        /*========================
         * FILL OUT MAT PATTERNS =
         *========================*/
        // Find Out Where Each Pattern Goes
        let pos = spiral_walker(mat_dim);
    
        // Check Spiral
        // let answer = spiral_walker(mat_dim);
        // for item in answer {
        //     println!("{:?}", item);
        // }
    
        // Iterate over each input byte.
        for (idx, &ele) in ideal_input.iter().enumerate() {
            //print!("{} ", ele);
            // Map each input byte to correct image.
            let image_chunk = images[ele as usize];
            // Fill out image pixels.
            for (i, &pixel) in image_chunk.iter().enumerate() {
                let x = (i % self.char_height as usize) as u32;
                let y = (i / self.char_width as usize) as u32;
                let mut rgb_val = pixel.0;
    
                if rgb_val[0] != 0xFF {
                    rgb_val[0] = color1;
                } else {
                    rgb_val[0] = !color1;
                }
                if rgb_val[1] != 0xFF {
                    rgb_val[1] = color2;
                } else {
                    rgb_val[1] = !color2;
                }
                if rgb_val[2] != 0xFF {
                    rgb_val[2] = color3;
                } else {
                    rgb_val[2] = !color3;
                }
                img_buffer.put_pixel(x+pos[idx].0*self.char_width, y+pos[idx].1*self.char_height, Rgb(rgb_val));
            }
        }
    
        // Save the ImageBuffer as an image file (e.g., PNG)
        let file_path = format!("{}.png", name);
        img_buffer.save(file_path).expect("Failed to save image chunk");
    }

}


