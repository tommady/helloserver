FROM rust:latest AS builder

RUN rustup target add aarch64-unknown-linux-musl

RUN USER=root cargo new --bin app
WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./
RUN rm src/*.rs
COPY ./src ./src

# RUN mkdir ~/.cargo
# RUN echo '[target.aarch64-unknown-linux-musl]\n\
# linker = "aarch64-linux-gnu-gcc"\n\
# rustflags = [ "-C", "target-feature=-crt-static", "-C", "link-arg=-lgcc" ]\n' >> ~/.cargo/config

# RUN cargo build --release --target aarch64-unknown-linux-musl
RUN cargo rustc --release --target aarch64-unknown-linux-musl -- -C link-arg=-lgcc

FROM busybox:latest AS runtime
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/rust ./

ENTRYPOINT [ "./rust" ]
EXPOSE 8080
