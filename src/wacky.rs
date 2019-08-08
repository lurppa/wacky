extern crate image;
extern crate glob;

use image::{GenericImageView, ImageBuffer};

pub mod filters;

pub fn wacky(image_dir: String, filter: fn(filters::Point) -> f32, buffer_size: usize) {
    let mut buffer: Vec<image::DynamicImage> = Vec::new();

    for (index, file_path) in glob::glob(&format!("{}*.png", image_dir)).unwrap().enumerate() {
        let file_name = file_path.unwrap();
        let frame = image::open(file_name.clone()).unwrap();
        let dimensions = frame.dimensions();
        let mut new_frame = ImageBuffer::new(dimensions.0, dimensions.1);
        
        if buffer.len() > buffer_size + 1 {
            buffer.remove(0);
        }

        buffer.push(frame);

        for y in 0..dimensions.1 {
            for x in 0..dimensions.0 {
                let normalized_point = filters::Point {
                    x: x as f32 / dimensions.0 as f32,
                    y: y as f32 / dimensions.1 as f32,
                };
                let filter_value = filter(normalized_point);

                let buffer_index: usize;
                if buffer.len() <= 1 {
                    buffer_index = 1;
                } else {
                    buffer_index = (filter_value * (buffer.len() - 2) as f32) as usize + 1
                }

                let pixel = buffer.get(buffer.len() - buffer_index).unwrap()
                        .get_pixel(x, y);
                new_frame.put_pixel(x, y, pixel);
            }
        }
        
        println!("frame {}", index);
        new_frame.save(file_name).unwrap();
    }
}