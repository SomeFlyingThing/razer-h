# razer-h

A simple lightweight Rust tool for setting mouse DPI and polling rate using the OpenRazer protocol.

## What it does

`razer-h` talks directly to a supported Razer mouse and lets you change:

- DPI
- Polling rate

The goal is to provide a small and simple alternative to heavier background tools when you only want to set basic mouse settings.

## Features

- Written in Rust
- Lightweight CLI tool
- Set mouse DPI
- Set mouse polling rate
- Uses OpenRazer protocol knowledge
- No heavy GUI
- Simple and direct

## Build

```bash
cargo build --release