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

# Copy the local (host) source code into the current directory in the image (/substrate)
COPY . .

# Install Rust, add it to the path, and install dependencies
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    rustup toolchain install nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly && \
    cargo build --release -j2  # Limit to 2 concurrent jobs

# Install sccache and set it as the Rust compiler wrapper
RUN cargo install sccache
ENV RUSTC_WRAPPER=sccache

# Start a new build stage, so that the final image only contains what we actually need to run the new Substrate node
FROM phusion/baseimage:0.11

# Expose the necessary port(s) if required (e.g., port 9944)
EXPOSE 9933
EXPOSE 9944
EXPOSE 9615
EXPOSE 9615

# Define the command to run your Substrate node with the --dev flag
CMD ["./target/release/omnichain-prototype", "--dev"]