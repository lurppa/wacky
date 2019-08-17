extern crate image;
extern crate glob;

use image::{GenericImageView, ImageBuffer};

pub mod filters;

pub struct Wacky {
    image_dir: String,
    filter_name: String,
    buffer_size: usize,
    verbose: bool,
}

impl Wacky {
    pub fn new() -> Wacky{
        Wacky {
            image_dir: String::from("tmp/"),
            filter_name: String::from("default"),
            buffer_size: 30,
            verbose: false,
        }
    }

    pub fn set_image_dir(&mut self, dir: String) -> &mut Wacky {
        self.image_dir = dir;
        self
    }

    pub fn set_filter(&mut self, filter: String) -> &mut Wacky {
        self.filter_name = filter;
        self
    }

    pub fn set_buffer_size(&mut self, size: usize) -> &mut Wacky {
        self.buffer_size = size;
        self
    }

    pub fn set_verbose(&mut self, value: bool) -> &mut Wacky{
        self.verbose = value;
        self
    }

    pub fn process(&mut self) {
        let mut buffer: Vec<image::DynamicImage> = Vec::new();

        for (index, file_path) in glob::glob(&format!("{}*.png", self.image_dir)).unwrap().enumerate() {
            let file_name = file_path.unwrap();
            let frame = image::open(file_name.clone()).unwrap();
            let dimensions = frame.dimensions();
            let mut new_frame = ImageBuffer::new(dimensions.0, dimensions.1);
            
            let filter = filters::get_filter(&self.filter_name);

            if buffer.len() > self.buffer_size + 1 {
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
            
            if self.verbose {
                println!("frame {}", index);
            }
            new_frame.save(file_name).unwrap();
        }
    }
}