FROM paritytech/ci-linux:production as builder
LABEL description="Substrate node"
WORKDIR /substrate

# Install Substrate dependencies
COPY . .
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    export PATH="$PATH:$HOME/.cargo/bin" && \
    rustup toolchain install nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly && \
    cargo build --release

FROM phusion/baseimage:0.11
LABEL maintainer="support@parity.io" description="Substrate node"
COPY --from=builder /substrate/target/release/node-template /usr/local/bin
RUN mv /usr/local/bin/node-template /usr/local/bin/substrate-node
EXPOSE 9933 9944 30333
VOLUME ["/substrate"]
ENTRYPOINT ["/usr/local/bin/substrate-node"]
