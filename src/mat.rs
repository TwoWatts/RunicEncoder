use rand::Rng;
use crypto_hash::{Algorithm, hex_digest};
use image::{DynamicImage, GenericImageView, Rgb};
use image::{ImageBuffer};
use std::fs;

fn ix2d(ele_per_row: u32, x: u32, y: u32) -> usize {
    return ( (ele_per_row as usize) * ((y) as usize)) + ((x) as usize);
}

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
    dim: u32,
    input: String,
    name: String,
    primary: (u8, u8, u8),
    secondary: (u8, u8, u8)
}

impl Mat {

    pub fn new(char_width: u32, char_height: u32, border_width: u32) -> Self {
        // Parse font characters as grid.
        let grid: Vec<Rgb<u8>> = load_image_grid("res/base16-mat.png", char_width + 2 * border_width, char_height + 2 * border_width, border_width);

        let mat = Mat {
            font: grid,
            char_width: char_width,
            char_height: char_height,
            dim: 0,
            input: "".to_string(),
            name: "".to_string(),
            primary: (0, 0, 0),
            secondary: (0, 0, 0),
        };
        mat
    }

    pub fn read(path: &String, char_width: u32, char_height: u32, border_width: u32) -> Self {
        let mut m = Mat::new(char_width, char_height, border_width);
        let buf = image::open(path).unwrap().into_rgb8();
        let (width, height) = buf.dimensions();
        let pri_color = buf.get_pixel(char_width / 2, char_height / 2);
        let bgd_color = buf.get_pixel(1, 2).0;
        m.dim = ((width as f32 / char_width as f32).ceil()) as u32;
        m.primary = (pri_color[0], pri_color[1], pri_color[2]);
        m.secondary = (bgd_color[0], bgd_color[1], bgd_color[2]);

        let answer = m.decode(path.as_str());
        println!("Decoded Text: \"{}\".", String::from_utf8_lossy(&answer).trim());
        m
    }

    pub fn get_input(&self) -> &String {
        return &self.input;
    }

    pub fn input(&mut self, input: String) {
        // Input
        self.input = input;

        // Name
        self.name = self.input.replace(" ", "_");
        self.name.push_str(".rune");

        // Mat Dimension
        self.dim = (((self.input.len()) as f32).sqrt().ceil()) as u32;

        // Color
        let (color1, color2, color3) = pick_color(self.input.as_bytes());
        self.primary.0 = color1;
        self.primary.1 = color2;
        self.primary.2 = color3;
        self.secondary.0 = !color1;
        self.secondary.1 = !color2;
        self.secondary.2 = !color3;
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn export(&mut self, name: &str) {
        if self.input == "" {
            println!("No input provided to Mat \"{}\".", name);
            return;
        }
        let input = self.input.as_bytes();

        // The Mat
        let mat_dim = self.dim;
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
    
        // Output Image
        let mut img_buffer = ImageBuffer::new(mat_pixel_width as u32, mat_pixel_height as u32);
    
        /*========================
         * FILL OUT MAT PATTERNS =
         *========================*/
        // Find Out Where Each Pattern Goes
        let trav_path = spiral_walker(mat_dim);
    
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
    
                if rgb_val[0] == 0xFF {
                    rgb_val[0] = self.primary.0;
                } else {
                    rgb_val[0] = self.secondary.0;
                }
                if rgb_val[1] == 0xFF {
                    rgb_val[1] = self.primary.1;
                } else {
                    rgb_val[1] = self.secondary.1;
                }
                if rgb_val[2] == 0xFF {
                    rgb_val[2] = self.primary.2;
                } else {
                    rgb_val[2] = self.secondary.2;
                }
                img_buffer.put_pixel(x+trav_path[idx].0*self.char_width, y+trav_path[idx].1*self.char_height, Rgb(rgb_val));
            }
        }
    
        // Save the ImageBuffer as an image file (e.g., PNG)
        let file_path = format!("{}.png", name);
        img_buffer.save(file_path).expect("Failed to save image chunk");
        println!("Rune Succeeded! See \"{}.png\"", name);
    }
    
    pub fn decode(&mut self, path: &str) -> Vec<u8> {

        println!("Decoding {}...", path);
        //println!("Char width = {}, Char height = {}", self.char_width, self.char_height);
        let mut decoded_input: Vec<u8> = Vec::new(); 

        // Open the file that needs decoding.
        // Read file as an array of pixels.
        let filepath = path.to_string();
        let buf = image::open(filepath).unwrap().into_rgb8();
        let mut source: Vec<Rgb<u8>> = Vec::new();
        
        // Find the order in which mat patterns are read.
        let trav_path: Vec<(u32, u32)> = spiral_walker(self.dim);

        for (sector_x, sector_y) in trav_path.iter() {
            let start_x = sector_x * self.char_width;
            let start_y = sector_y * self.char_height;
            let end_x = start_x + self.char_width;
            let end_y = start_y + self.char_height;
            for y in start_y..end_y {
                for x in start_x..end_x {
                    let pixel = buf.get_pixel(x, y).0;
                    source.push(Rgb(pixel));
                }
            }
        }

        // Make array of addresses for image chunks.
        let pixels_per_image: usize = (self.char_width * self.char_height) as usize;
        let src_img_chunks: Vec<&[Rgb<u8>]> = source.chunks(pixels_per_image).collect();

        // Iterate over image.
        let primary_rgb = self.primary;
        let background_rgb = self.secondary;

        for i in 0..trav_path.len() {
            let mut left_hex: u8 = 0;
            let mut right_hex: u8 = 0;
            let image_chunk = src_img_chunks[i];
            let l_one = image_chunk[ix2d(self.char_width, 1, 0)].0;
            let l_two = image_chunk[ix2d(self.char_width, 1, 1)].0;
            let l_four = image_chunk[ix2d(self.char_width, 1, self.char_height-1-1)].0;
            let l_eig = image_chunk[ix2d(self.char_width, 1, self.char_height-1)].0;
            let r_one = image_chunk[ix2d(self.char_width, self.char_width-1-1, 0)].0;
            let r_two = image_chunk[ix2d(self.char_width, self.char_width-1-1, 1)].0;
            let r_four = image_chunk[ix2d(self.char_width, self.char_width-1-1, self.char_height-1-1)].0;
            let r_eig = image_chunk[ix2d(self.char_width, self.char_width-1-1, self.char_height-1)].0;
    
            // for (idx, item) in image_chunk.iter().enumerate() {
            //     let item_ix = item.0;
            //     if ((idx % self.char_width as usize) == 0) {
            //         println!("");
            //     }
            //     if item_ix[0] == self.primary.0 && item_ix[1] == self.primary.1 && item_ix[2] == self.primary.2 {
            //         print!("■");
            //     }
            //     else {
            //         print!("□");
            //     }
            // }
            // println!("");

            if l_one[0]  == primary_rgb.0 && l_one[1] == primary_rgb.1 && l_one[2] == primary_rgb.2 { left_hex += 1; }
            if l_two[0]  == primary_rgb.0 && l_two[1] == primary_rgb.1 && l_two[2] == primary_rgb.2 { left_hex += 2; }
            if l_four[0] == primary_rgb.0 && l_four[1] == primary_rgb.1 && l_four[2] == primary_rgb.2 { left_hex += 4; }
            if l_eig[0]  == primary_rgb.0 && l_eig[1] == primary_rgb.1 && l_eig[2] == primary_rgb.2 { left_hex += 8; }
            //println!("left = {}", left_hex);
            if r_one[0]  == primary_rgb.0 && r_one[1] == primary_rgb.1 && r_one[2] == primary_rgb.2 { right_hex += 1; }
            if r_two[0]  == primary_rgb.0 && r_two[1] == primary_rgb.1 && r_two[2] == primary_rgb.2 { right_hex += 2; }
            if r_four[0] == primary_rgb.0 && r_four[1] == primary_rgb.1 && r_four[2] == primary_rgb.2 { right_hex += 4; }
            if r_eig[0]  == primary_rgb.0 && r_eig[1] == primary_rgb.1 && r_eig[2] == primary_rgb.2 { right_hex += 8; }
            //println!("right = {}", right_hex);
            decoded_input.push((left_hex << 4) | right_hex);
        }
        //println!("{:?}", decoded_input);
        decoded_input
    }

}


