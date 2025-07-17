# ---- Stage 1: Build ----
# We use the official Rust image as a builder.
# The "slim" variant is used to keep the image size down.
FROM rust:1-slim as builder

# Install dependencies needed for a static build
WORKDIR /usr/src/app

# Create a new empty shell project.
# This is a bit of a trick to cache dependencies.
RUN cargo init

# Copy over your manifests
COPY Cargo.toml Cargo.lock ./

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the rest of your application's source code
COPY src ./src

# Build the application
# This will be much faster since the dependencies are already cached
RUN rm ./target/release/deps/chatgptassistanse*
RUN cargo build --release

# ---- Stage 2: Runtime ----
# Use a minimal base image for the final container.
# "slim-bullseye" is a good choice for a small footprint with basic tools.
FROM debian:slim-bullseye

# Copy the compiled binary from the builder stage.
# The package name in Cargo.toml is "ChatgptAssistanse", so the binary will have this name.
COPY --from=builder /usr/src/app/target/release/ChatgptAssistanse /usr/local/bin/app

# Expose the port the app runs on.
EXPOSE 3000

# Set the entrypoint for the container.
# The application will be run when the container starts.
CMD ["/usr/local/bin/app"] 