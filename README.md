# Storm Backend

A lightweight Rust backend service for payment processing and [umami](https://umami.is/) analytics integration.

## Features

- **Actix-web** server for fast HTTP handling
- **Stripe** webhook processing
- **Analytics** integration
- Environment-based configuration

## Getting Started

### Requirements

- Rust (see `rust-toolchain.toml`)
- Required environment variables:
  - `STRIPE_SECRET`
  - `ANALYTICS_WEBSITE_ID`
  - `ANALYTICS_API_URL`

### Building

```bash
cargo build
```

### Running

```bash
cargo run -- --port 8080 --bind-address 127.0.0.1
```

**Options:**
- `--port` (default: 8080)
- `--bind-address` (default: 127.0.0.1)

### With Nix

```bash
nix build
```

## Project Structure

```
src/
├── main.rs      # Server setup and configuration
└── routes/      # API endpoints
```
