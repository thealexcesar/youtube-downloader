from pytube import YouTube
import subprocess
import os

def download_and_convert(url, output_path="."):
    try:
        yt = YouTube(url)
        video_stream = yt.streams.filter(file_extension="mp4").first()

        output_filename = yt.title.replace(" ", "_") + ".mp3"
        audio_file_path = os.path.join(output_path, output_filename)

        subprocess.run(['ffmpeg', '-i', video_stream.url, audio_file_path])

        return audio_file_path
    except Exception as e:
        return f"Erro durante a conversão: {e}"

if __name__ == "__main__":
    video_url = input("enter youtube URL: ")
    output_path = input("Choose your folder (Default: ~/Music): ") or "~/Music"

    output_path = os.path.expanduser(output_path)
    output_path = os.path.expanduser(os.path.join("~/", output_path))

    if not os.path.exists(output_path):
        os.makedirs(output_path)

    audio_path = download_and_convert(video_url, output_path)

    if audio_path.startswith("Error during conversion."):
        print(audio_path)
    else:
        print(f"Success downloaded: {audio_path}")