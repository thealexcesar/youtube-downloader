FROM rust:latest

RUN apt-get update && apt-get install -y python3-pip
RUN pip3 install --upgrade yt-dlp

WORKDIR /usr/src/app
COPY . .

CMD ["cargo", "run"]