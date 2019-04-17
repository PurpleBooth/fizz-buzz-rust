FROM liuchong/rustup:1.34.0-musl as build
ENV PORT=8080
RUN rustup component add rustfmt --toolchain 1.34.0-x86_64-unknown-linux-gnu

WORKDIR /usr/src/myapp
COPY . .
RUN cargo fmt -- --check
RUN cargo test
RUN cargo build --release
RUN cargo install --path .

FROM scratch
ENV PORT=8080

COPY --from=build /root/.cargo/bin/fizz-buzz-rust /fizz-buzz-rust

ENTRYPOINT ["/fizz-buzz-rust"]
