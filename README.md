<div align="center">

# Termtalk

**Lightweight, end-to-end encrypted terminal chat written in Rust.**

`Rust` • `Tokio` • `Ratatui` • `ChaCha20-Poly1305`

<br>

<img src="https://github-production-user-asset-6210df.s3.amazonaws.com/93241730/705518042-84e27b51-5cab-4242-b381-19ed7e632965.gif?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20261113%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20261113T191247Z&X-Amz-Expires=300&X-Amz-Signature=6d703dd371378a43dfe2459fa8a8cd11610e04658cc59fa1e3cd4eb422118e17&X-Amz-SignedHeaders=host&response-content-type=image%2Fgif" alt="Termtalk Demo" width="1200">

</div>

---

## Overview

**Termtalk** is a fast, distraction-free terminal messaging client designed around privacy and minimalism. It uses a "dumb-relay" WebSocket architecture combined with client-side symmetric encryption, allowing users to create isolated, password-protected chat rooms on a single shared server without complex room management or accounts.

## Key Features

- **End-to-End Encryption (E2EE):** All messages are encrypted locally before leaving the client using **ChaCha20-Poly1305** with a unique random 12-byte nonce per message. The server acts purely as a blind relay and cannot read message contents.
- **Zero-Config Virtual Rooms:** The secret key passed at startup acts as a cryptographic room identifier. Only participants sharing the exact same key can decrypt and view each other's messages; unauthorized observers see nothing.
- **Responsive Terminal UI:** Built with **Ratatui**, featuring dynamic word-wrapping on terminal resize, automatic scrolling to new messages, and keyboard history navigation.
- **Non-Blocking Architecture:** Network I/O (`Tokio` + `tokio-tungstenite`) is completely decoupled from the 60 FPS UI render loop via asynchronous `mpsc` channels.

## Installation & Usage

### Prerequisites

- [Rust and Cargo](https://rustup.rs) installed on your machine.
- A running instance of the Termtalk WebSocket relay server.

### Build from Source

1. Clone the repository:

   ```bash
   git clone https://github.com
   cd termtalk
   ```

2. Build the optimized release binary:

   ```bash
   cargo build --release
   ```

3. Run the client:
   ```bash
   ./target/release/termtalk <HOST> --username <NAME> --key <SECRET>
   ```

### Example

```bash
./target/release/termtalk 127.0.0.1 -u Sergey -k my_secret_room
```

### CLI Arguments

| Flag             | Argument     | Description                                                 |
| :--------------- | :----------- | :---------------------------------------------------------- |
| `<HOST>`         | IPv4 Address | Target WebSocket server IP address (e.g., `127.0.0.1`).     |
| `-p, --port`     | `<PORT>`     | Server port.                                                |
| `-u, --username` | `<NAME>`     | Display name in chat (validated: 2 to 20 characters).       |
| `-k, --key`      | `<SECRET>`   | Shared secret passphrase used to derive the encryption key. |
| `-h, --help`     | —            | Print help information.                                     |

## Keyboard Controls

- **`Enter`** — Encrypt and send message.
- **`Up` / `Down`** — Scroll through chat history (disables auto-scroll until reaching the bottom).
- **`Esc`** — Cleanly restore terminal state and exit.

## Security Architecture

1. **Key Derivation:** The user-provided passphrase is hashed using **SHA-256** to produce a 256-bit symmetric key.
2. **Encryption:** Serialized JSON events (`Join`, `Leave`, `UserMessage`) are encrypted via `ChaCha20-Poly1305`, prepended with a fresh CSPRNG nonce, and Base64-encoded for WebSocket transport.
3. **Verification:** Incoming payloads with an invalid authentication tag (wrong key or tampered data) are silently dropped on the client side.
