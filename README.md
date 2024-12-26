# evm-prometheus-exporter
Yet another EVM exporter - A Prometheus exporter for Ethereum Virtual Machine chains

## Overview

This is a simple prometheus exporter for EVM chains, written in Rust. Opinionated, but simple.

## Features

- Exports number of accounts on the network
- Uses `alloy` for EVM interactions
- Lightweight and efficient

## Usage

```bash
# Run with default settings (connects to http://localhost:8545)
evm-prometheus-exporter

# Run with custom RPC endpoint
evm-prometheus-exporter --rpc-url https://your-node:8545

# Run with custom metrics port (default: 9184)
evm-prometheus-exporter --metrics-port 9185
```

## Metrics

| Metric Name | Description | Type |
|-------------|-------------|------|
| evm_total_accounts | Total number of accounts on the network | Gauge |

## Building

```bash
cargo build --release
```

## License

MIT License