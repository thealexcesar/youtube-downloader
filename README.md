# README: Rust YouTube Downloader

## This repository contains a Rust script for downloading audios and videos from YouTube using `yt-dlp`. Easy to use, you can grab your favorite YouTube content in your desired format (mp3 or mp4) and save it to your `HOME/Music` directory.

## Requirements

- Operating System: Debian-based Linux (recommended), Windows, or macOS.
- `yt-dlp` tool installed and available in the system PATH.

## Installation and Usage

### Debian-based Linux:

1. First, let's install [yt-dlp](https://github.com/yt-dlp/yt-dlp):
    ```
    sudo apt-get update
    sudo apt-get install python3-pip
    pip3 install --upgrade yt-dlp
    ```

2. Clone the repository and navigate into the directory:
    ```
    git clone https://github.com/thealexcesar/youtube-downloader.git
    cd https://github.com/thealexcesar/youtube-downloader.git
    ```

3. Compile and run the script:
    ```
    cargo run
    ```
   - Enter the YouTube URL.
   - Choose the desired format (mp3 or mp4).
   - The file will be downloaded to your media directory.
---

If you encounter any issues or have suggestions, feel free to open an issue or a pull request.
