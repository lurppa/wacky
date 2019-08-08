extern crate image;
extern crate argparse;

mod ffmpeg;
mod wacky;

fn main() {
    let input_file = String::from("res/input.mp4");
    let output_file = String::from("res/output.mp4");
    let temp_dir = String::from("res/tmp/");
    ffmpeg::explode_video(input_file, temp_dir.clone(), None);
    wacky::wacky(temp_dir.clone(), wacky::filters::radial, 60);
    ffmpeg::construct_video(temp_dir.clone(), output_file);
    ffmpeg::clean(temp_dir);
}
