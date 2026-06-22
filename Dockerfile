# Use the latest Rust stable release as base image
FROM rust:1.94

# cd app
# If the app folder does not exist, it will be created for us by Docker
WORKDIR /app

# Install linkers and dependencies (if they are missing)
RUN apt update && apt install lld clangd -y

# Copy all the files from working environment to our Docker image
COPY . .

ENV SQLX_OFFLINE true

# Build the binary with the relase profile to make it fast and slim
RUN cargo build --release

# When `docker run` is executed, launch the binary
ENTRYPOINT ["./target/release/zero2prod_rs"]
