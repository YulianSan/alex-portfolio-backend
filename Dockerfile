FROM rust:1.78

WORKDIR /usr/src/backend

COPY . .
ENV CARGO_TARGET_DIR=/usr/src/backend/build
RUN cargo build
RUN cargo install cargo-watch

ENTRYPOINT [ "cargo", "watch", "-x", "run" ]
