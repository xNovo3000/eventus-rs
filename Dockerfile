# Pull builder image
FROM rust:1.74.0-slim-bookworm as builder
# Copy required folders into workspace
WORKDIR /root
COPY src/ src/
COPY templates/ templates/
COPY Cargo.toml Cargo.toml
COPY diesel.toml diesel.toml
# Clean and build
RUN cargo build --release

# Pull container image
FROM redhat/ubi9-minimal:9.3-1361.1699548032
# Create user and group and switch
RUN microdnf install -y shadow-utils
RUN groupadd -g 500 webapp
RUN useradd -u 500 -g webapp -M webapp
RUN microdnf remove -y shadow-utils
RUN microdnf clean all
# Create working directory and assign to webapp
RUN mkdir /opt/app
RUN chown -R webapp:webapp /opt/app
# Switch user and set workdir
USER webapp
WORKDIR /opt/app
# Create required folders
RUN mkdir /opt/app/config
RUN mkdir /opt/app/logs
# Copy all required files
COPY --from=builder --chown=webapp:webapp /root/target/release/eventus-rs eventus-rs
COPY --chown=webapp:webapp static/ static/
# Launch
ENTRYPOINT /opt/app/eventus-rs