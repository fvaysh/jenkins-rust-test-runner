FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build
CMD ["cargo", "test"]
