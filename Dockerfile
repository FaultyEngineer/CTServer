FROM rust:latest

WORKDIR /usr/src/ctserver
COPY . .
RUN cargo build --release

EXPOSE 8080
CMD ["./target/release/ctserver"]