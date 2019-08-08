use std::process::Command;
use std::fs;

pub fn dir_format(dir: &mut String) {
    if let Some(character) = dir.get((dir.len() - 1)..){
        if character != "/" {
            dir.push('/');
        }
    }
}

pub fn explode_video(source: String, dest_dir: String, resolution: Option<String>) {
    fs::create_dir_all(&dest_dir)
            .expect("failed to create destination directory");
    
    let mut resolution_command: Vec<String> = Vec::new();
    if let Some(resolution_string) = resolution {
        resolution_command.push(String::from("-vf"));
        resolution_command.push(format!("scale={}", resolution_string));
    }

    Command::new("ffmpeg")
            .arg("-y")
            .arg("-i")
            .arg(source)
            .args(&resolution_command)
            .arg(format!("{}output_%04d.png", dest_dir))
            .output()
            .expect("failed to execute ffmpeg");
}

pub fn construct_video(source_dir: String, output: String) {
    Command::new("ffmpeg")
            .arg("-y")
            .arg("-i")
            .arg(format!("{}output_%04d.png", source_dir))
            .arg(output)
            .output()
            .expect("failed to execute ffmpeg");
}

pub fn clean(dir: String) {
    fs::remove_dir_all(dir).unwrap();
}