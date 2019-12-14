FROM rust:latest

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

CMD ["station-emulator"]
