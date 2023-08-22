# Substrate Node Dockerfile Documentation

This Markdown documentation covers how to build and run a Docker container using the provided Dockerfile. The Dockerfile is designed to create a Substrate node environment. Substrate is a blockchain framework that provides the necessary building blocks to construct a blockchain.

## Understanding the Dockerfile

Before proceeding to the build and run steps, it's crucial to understand what the Dockerfile is doing.

### Sections:

1. **Base Image**: `FROM paritytech/ci-linux:production as builder` uses Parity's CI Linux as the base image.
2. **Metadata**: `LABEL description="Substrate node"` adds metadata to the Docker image.
3. **Setting Work Directory**: `WORKDIR /substrate` sets the working directory inside the container.
4. **Installing Dependencies**: Installs Rust and the necessary toolchains for Substrate.
5. **Creating Final Image**: `FROM phusion/baseimage:0.11` sets up the final image.
6. **Copying Files**: Copies the built Substrate node binary to `/usr/local/bin`.
7. **Port and Volume**: Exposes necessary ports and sets up a volume.
8. **Entrypoint**: Defines the entry point of the container as `/usr/local/bin/substrate-node`.

---

## Pre-requisites

- Install Docker on your system. You can find the installation guide on the [official Docker website](https://docs.docker.com/get-docker/).

## Building the Docker Image

1. **Navigate to Dockerfile Directory**: Navigate to the directory containing the Dockerfile in your terminal.

    ```
    cd /path/to/directory
    ```

2. **Build Image**: Run the following command to build the Docker image. Replace `substrate-node-image` with the name you want to give to the built image.

    ```bash
    docker build -t substrate-node-image .
    ```

   The `-t` flag tags the image with a name, and the `.` indicates that the build context is the current directory.

## Running the Docker Container

After building the image, you can run a container from it using the following command:

```bash
docker run -p 9933:9933 -p 9944:9944 -p 30333:30333 -v $(pwd)/substrate-data:/substrate substrate-node-image
