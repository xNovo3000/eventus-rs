FROM rust:1.73.0-slim-bookworm as builder
WORKDIR /workspace
COPY /src /workspace/src
COPY /templates /workspace/templates
COPY /Cargo.toml /workspace/Cargo.toml
RUN cargo build --release

FROM redhat/ubi9-micro:9.2-15.1696515526
WORKDIR /opt/app
RUN mkdir logs
COPY /static static
COPY --from=builder /workspace/target/release/eventus-rs eventus-rs
VOLUME /opt/app/logs
EXPOSE 3000
ENTRYPOINT /opt/app/eventus-rs