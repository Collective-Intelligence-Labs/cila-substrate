# Use Parity's CI Linux image as the builder
FROM paritytech/ci-linux:production as builder

# Metadata as described earlier
LABEL description="Substrate node"

# Work in the /substrate directory within the Docker image
WORKDIR /substrate

# Set environment variables for consistent Rust and Cargo setup
ENV CARGO_HOME=/substrate/cargo
ENV RUSTUP_HOME=/substrate/rustup
ENV PATH="$PATH:/substrate/cargo/bin"

# Install sccache and set it as the Rust compiler wrapper
RUN cargo install sccache
ENV RUSTC_WRAPPER=sccache

# Copy the local (host) source code into the current directory in the image (/substrate)
COPY . .

# Install Rust, add it to the path, and install dependencies
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    rustup toolchain install nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly && \
    cargo build --release -j2  # Limit to 2 concurrent jobs

# Start a new build stage, so that the final image only contains what we actually need to run the new Substrate node
FROM phusion/baseimage:0.11
