FROM rust:latest #as builder
WORKDIR /usr/src/day02_part2
COPY . .
RUN cargo install --bin part_two --path .

# FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/local/cargo/bin/day02_part2 /usr/local/bin/day02_part2
# CMD ["day02_part2"]

# docker run --rm --it /bin/bash --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/day02_part2 -w /usr/src/day02_part2 rust:latest cargo build --release