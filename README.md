<div align="center">

# Termtalk

**Lightweight, end-to-end encrypted terminal chat written in Rust.**

`Rust` • `Tokio` • `Ratatui` • `ChaCha20-Poly1305`

<br>

<img src="https://github.com/user-attachments/assets/89199d29-824f-49e8-87b5-73025782dc52" width="1200" alt="Termtalk Demo" />

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
   git clone https://github.com/NVinox/termtalk.git
   cd termtalk
   ```

2. Build the optimized release binary:

   ```bash
   cargo build --release
   ```

3. Run the client:
   ```bash
   ./target/release/termtalk <HOST> --port <PORT> --username <NAME>
   ```

### Example

```bash
./target/release/termtalk 127.0.0.1 -p 8000 -u Sergey
```

### CLI Arguments

| Flag             | Argument     | Description                                             |
| :--------------- | :----------- | :------------------------------------------------------ |
| `<HOST>`         | IPv4 Address | Target WebSocket server IP address (e.g., `127.0.0.1`). |
| `-p, --port`     | `<PORT>`     | Server port.                                            |
| `-u, --username` | `<NAME>`     | Display name in chat (validated: 2 to 20 characters).   |
| `-h, --help`     | —            | Print help information.                                 |

## Keyboard Controls

- **`Enter`** — Encrypt and send message.
- **`Up` / `Down`** — Scroll through chat history (disables auto-scroll until reaching the bottom).
- **`Esc`** — Cleanly restore terminal state and exit.

## Security Architecture

1. **Key Derivation:** The user-provided passphrase is processed using **Argon2** (the default variant) with a fixed salt to produce a 256-bit symmetric key.
2. **Limitation:** The current implementation uses a fixed salt and a simplified KDF configuration. While sufficient for a private chat room, it is not audited for enterprise-grade security.
3. **Encryption:** Serialized JSON events (`Join`, `Leave`, `UserMessage`) are encrypted via `ChaCha20-Poly1305`, prepended with a fresh CSPRNG nonce, and Base64-encoded for WebSocket transport.
4. **Verification:** Incoming payloads with an invalid authentication tag (wrong key or tampered data) are silently dropped on the client side.
