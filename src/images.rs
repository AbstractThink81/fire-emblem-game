use image::{ImageBuffer, DynamicImage, Rgb};

pub fn save_grid_image(grid_size: u32, outline_thickness: u32, path: &str) {
    let image = generate_grid_image(grid_size, outline_thickness);
    image.save(path).unwrap();
}

pub fn generate_grid_image_dynamic(grid_size: u32, outline_thickness: u32) -> DynamicImage {
    let image = generate_grid_image(grid_size, outline_thickness);
    DynamicImage::ImageRgb8(image)
}

pub fn generate_grid_image(grid_size: u32, outline_thickness: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = grid_size;
    let height = grid_size;

    let mut image = ImageBuffer::new(width, height);

    let black = Rgb([0, 0, 0]);
    let blue = Rgb([0, 0, 255]);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if x < outline_thickness  || y < outline_thickness || x > width - outline_thickness || y > height - outline_thickness {
            *pixel = black;
        } else {
            *pixel = blue;
        }
    }

    // Return the image
    image
}