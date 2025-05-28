# microbit-sand

Simple sand simulation for the [BBC micro:bit v2](https://microbit.org/).

The micro:bit v2 is a small, low-cost microcontroller board designed for education. It features a 5x5 LED matrix,
buttons, and various sensors. This project demonstrates how to create a simple sand simulation using the micro:bit's
hardware capabilities.

The micro:bit v2 has a [nRF52833](https://www.nordicsemi.com/Products/nRF52833) chip with a Cortex-M4F core, which is
capable of running Rust code.

## Flashing the micro:bit

### Initial setup

1. Install the `probe-rs` tool:
   https://probe.rs/docs/getting-started/installation/
   ```bash
   cargo install probe-rs-tools
   probe-rs chip list | grep nrf52833
   ```
2. Linux users: modify udev rules so you don't need to use sudo to access it.. Please check
   out: https://docs.rust-embedded.org/discovery/microbit/03-setup/linux.html#udev-rules

### Building and flashing

Connect your micro:bit to your computer via USB.

```
cargo embed
```

## Inspecting the binary

```
cargo install cargo-binutils
rustup component add llvm-tools
cargo size -- -Ax
```

## How we got here

https://www.youtube.com/@therustybits has a great video playlist on how to start with embedded rust development on a
micro:bit. We started with https://www.youtube.com/watch?v=TOAynddiu5M

There's also a microbit v2 discovery guide in the book on embedded rust
development: https://docs.rust-embedded.org/discovery-mb2/
