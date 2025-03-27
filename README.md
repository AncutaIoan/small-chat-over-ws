# Small chat over websockes in Rust

This project implements a simple WebSocket server using the **Warp** web framework and **Tokio** for asynchronous runtime in Rust. The server allows multiple clients to connect and communicate via WebSockets. Messages sent by one client are broadcast to all other connected clients.

## Features
- WebSocket server using **Warp**.
- Broadcast messages to all connected clients.
- Asynchronous processing with **Tokio** and **Futures**.
- **Mutex** and **Arc** are used to share the broadcast sender between multiple WebSocket connections.

## Prerequisites

Before running the server, ensure that you have **Rust** installed on your machine.

- Install **Rust** from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
- Verify installation by running:

  ```bash
  rustc --version
  cargo --version
