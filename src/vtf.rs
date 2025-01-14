// program to convert videos into frame with appropriate error handling
use std::env;
use std::fs;
use std::process;

fn vtf(path: &String, output: &String) {
    if !fs::exists(&path).unwrap() {
        println!("{path} - could not be found!");
        return;
    }

    // create the full path for the video frames folder
    let output_path = format!(
        "{}/{}-frames",
        output,
        path.split('/').last().unwrap().split('.').nth(0).unwrap()
    );
    // Remove the directory if it already exists
    if fs::exists(&output_path).unwrap() {
        println!("Removing the existing : {} directory", output_path);
        match fs::remove_dir_all(&output_path) {
            Ok(_) => {
                println!(
                    "successfully deleted the pre-existing : {} directory !",
                    output_path
                );
            }
            Err(_) => {
                println!(
                    "Failed to remove the pre-existing : {} directory !, skiping the video {}",
                    output_path, path
                );
                return;
            }
        }
    }
    match fs::create_dir_all(&output_path) {
        Ok(_) => {
            // ffmpeg -i ./assets/res/train/video1.mp4 -vf 'fps=5' ./assets/out/video1-frames/frame_%4d.jpg
            let proc = process::Command::new("ffmpeg")
                .args([
                    "-i",
                    &path,
                    "-vf",
                    "fps=5",
                    &format!("{}/frame_%4d.jpg", output_path),
                ])
                .output()
                .unwrap();
            println!("Executing ffmpeg for {output_path} :: {}", proc.status);
            if proc.status.success() {
                println!("ffmpeg : successfully covnverted video into frames");
            } else {
                println!("ffmpeg :: failed");
                let stderr = String::from_utf8_lossy(&proc.stderr);
                println!("error : {stderr}");
            }
        }
        Err(_) => {
            println!("Failed to create directory : {}", output_path);
            return;
        }
    };
}

fn main() {
    let objective = String::from(env::args().nth(1).unwrap());

    if &objective != "train" && &objective != "test" {
        println!("unknown object {{train | test}} : {}", objective);
        process::exit(1);
    }

    let video_root = String::from(format!("./assets/res/{}", objective));
    let output_root = String::from(format!("./assets/out/{}", objective));

    println!("objective : {objective}\nvideo root : {video_root} \noutput root : {output_root}");

    if let Ok(videos) = fs::read_dir(&video_root) {
        for entry in videos {
            if let Ok(video) = entry {
                let path = video.path().to_str().unwrap().to_string();
                vtf(&path, &output_root);
            } else {
                println!("failed to access entry : {:?}", entry.unwrap());
            }
        }
    } else {
        println!("could not read the directory : {}", video_root);
    }
}