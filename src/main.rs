extern crate image;
extern crate argparse;

mod ffmpeg;
mod wacky;

use argparse::{ArgumentParser, StoreTrue, Store};
use wacky::Wacky;

fn main() {
    let mut input_file = String::default();
    let mut output_file = String::default();
    let mut temp_dir = "tmp/".to_string();
    let mut filter = "default".to_string();
    let mut buffer_size = 30;
    let mut verbose = false;
    let mut resolution = String::default();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Set video frames to go back in time by pixel basis.");
        ap.refer(&mut input_file)
            .add_argument("input", Store, "Input file to be processed");
        ap.refer(&mut output_file)
            .add_argument("output", Store, "Output file name");
        ap.refer(&mut temp_dir)
            .add_option(&["-t", "--temp-dir"], Store, "Temporary file location");
        ap.refer(&mut filter)
            .add_option(&["-f", "--filter"], Store, "Filter name (one of 'default' or 'radial')");
        ap.refer(&mut buffer_size)
            .add_option(&["-b", "--buffer-size"], Store, "Buffer size in frames");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut resolution)
            .add_option(&["-r", "--resolution"], Store, "Output resolution");
        ap.parse_args_or_exit();
    }

    ffmpeg::explode_video(input_file, temp_dir.clone(),
        if resolution.is_empty() {
            None
        } else {
            Some(resolution)
        }
    );

    Wacky::new()
        .set_image_dir(temp_dir.clone())
        .set_filter(filter.clone())
        .set_buffer_size(buffer_size)
        .set_verbose(verbose)
        .process();

    ffmpeg::construct_video(temp_dir.clone(), output_file);
    ffmpeg::clean(temp_dir);
}
