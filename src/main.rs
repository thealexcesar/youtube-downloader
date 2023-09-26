//! Program for downloading audio or video from YouTube using yt-dlp.
//! Ensure you have `yt-dlp` installed and available in your system's PATH.
//! Version: 1.0.0
//! Author: alexcesar

use std::env;
use std::process::Command;
use std::io;

fn main() {
    let url = prompt_for_input("Enter YouTube URL:").trim().to_string();
    let video_id = extract_video_id(&url).expect("Invalid YouTube URL.");

    let format = prompt_for_input("Choose format (mp3/mp4):").trim().to_string();

    let status = Command::new("yt-dlp")
        .arg("--get-filename")
        .arg(format!("https://www.youtube.com/watch?v={}", video_id))
        .output()
        .expect("Failed to start process");

    let filename = String::from_utf8_lossy(&status.stdout).trim().to_string();
    let cleaned_filename = parameterize(&filename);

    download_video_or_audio(&video_id, &format, &cleaned_filename);
}

fn parameterize(input: &str) -> String {
    let cleaned: String = input.chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ')
        .collect();

    let parameterized: String = cleaned
        .trim()
        .to_lowercase()
        .replace(" ", "-");

    parameterized.chars().take(20).collect()
}

fn extract_video_id(url: &str) -> Option<&str> {
    url.split('=').nth(1)
}

fn download_video_or_audio(id: &str, format: &str, filename: &str) {
    let home = env::var("HOME").unwrap_or(".".to_string());
    let output_path = format!("{}/Music/{}", home, filename);

    match format {
        "mp3" => {
            Command::new("yt-dlp")
                .arg("-x")
                .arg("--audio-format")
                .arg("mp3")
                .arg("--output")
                .arg(&output_path)
                .arg(format!("https://www.youtube.com/watch?v={}", id))
                .spawn()
                .expect("Failed to start download process")
                .wait()
                .expect("Failed to wait for download process");
        },
        "mp4" => {
            Command::new("yt-dlp")
                .arg("-f")
                .arg("bestvideo+bestaudio/best")
                .arg("--output")
                .arg(&output_path)
                .arg(format!("https://www.youtube.com/watch?v={}", id))
                .spawn()
                .expect("Failed to start download process")
                .wait()
                .expect("Failed to wait for download process");
        },
        _ => println!("Invalid format choice.")
    }
}

fn prompt_for_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}
