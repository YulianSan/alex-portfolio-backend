FROM rust:1.78

ARG USER_ID=1000
RUN echo "create dev user with uid $USER_ID"
RUN useradd --create-home --uid $USER_ID sandev

WORKDIR /usr/src/backend

COPY . .
RUN chown -R sandev:sandev /usr/src/backend

USER sandev

ENV CARGO_TARGET_DIR=/usr/src/backend/build
RUN cargo install cargo-watch
RUN cargo install diesel_cli

ENTRYPOINT [ "cargo", "watch", "-x", "run" ]
