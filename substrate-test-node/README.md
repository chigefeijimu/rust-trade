# Substrate Test Node - Docker Setup

This README provides instructions on how to set up and run a Substrate test node using Docker.

## Prerequisites
Ensure that you have the following installed on your system:
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

## Getting Started

### 1. Build and Start the Substrate Test Node
Run the following command to build and start the Substrate node container in detached mode:

```bash
docker-compose up -d --build
```

### 2. Verify the Running Container
After the container starts, verify that it is running with:

```bash
docker ps
```

You should see an entry for `substrate-node` in the running containers list.

### 3. Connect to the Local Node

To interact with the local Substrate node, follow these steps:

1. Visit [Polkadot.js Apps](https://polkadot.js.org/apps/)
2. Click the network selector in the top left corner.
3. Select **DEVELOPMENT** -> **Local Node**.
4. Ensure the WebSocket URL is set to:
   ```
   ws://127.0.0.1:9944
   ```
5. Click **Switch** to connect to your local node.

## Dockerfile Explanation

The provided `Dockerfile`:

- Uses `ubuntu:20.04` as the base image.
- Sets the timezone to `Asia/Shanghai`.
- Installs required dependencies (`curl`, `wget`).
- Downloads and extracts `substrate-contracts-node` v0.35.0 from Parity’s GitHub releases.
- Moves the binary to `/usr/local/bin/` for execution.
- Defines `/substrate` as the working directory.
- Exposes ports `9944`, `9933`, and `30333` for external access.
- Runs the Substrate Contracts Node in development mode (`--dev --rpc-external`).

## Ports Explanation

- **9944**: WebSocket RPC port (used for interaction with tools like Polkadot.js Apps)
- **9933**: HTTP RPC port
- **30333**: P2P networking port

## Stopping and Restarting the Container

To stop the running container while keeping the data and configuration intact:

```bash
docker-compose stop
```

To start the container again without rebuilding:

```bash
docker-compose start
```

To completely stop and remove the container:

```bash
docker-compose down
```

To restart the container:

```bash
docker-compose up -d
```

## Volume Persistence
The container uses a named volume `substrate-data` to persist blockchain data across restarts.