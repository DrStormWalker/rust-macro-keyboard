# Rust Club - Raspberry Pi Pico macro keyboard

## Table of Contents

- [Table of Contents](#table-of-contents)
- [Supported Platforms](#supported-platforms)
- [Requirements](#requirements)
  - [NixOS](#nix-os)
  - [Development Dependencies](#development-dependencies)
  - [Installation of Development Dependencies](#installation-of-dependencies)
- [Running](#running)

## Supported Platforms

- Linux
- MacOS (Tested on Ventura 13.1)

NOTE: Other platforms may work but are not explicitely defined here

## Requirements

- A Raspberry Pi Pico

### NixOS

This project is packaged using nix flakes. A development environment
can be created using the following command in the root of the project:
```sh
nix develop
```
This will download all necessary build tools for use in a nix shell
environment.

### Development Dependencies

- [Rust tooling](https://rustup.rs)
- Rust toolchin support for cortex-m0+ processors (thumbv6m-none-eabi)
  (can be installed using rustup, see [here](#installation-of-dependencies))
- flip-link (installed using cargo, see [here](#installation-of-dependencies))
- elf2uf2-rs (instal, see [here](#installation-of-dependencies))

### Installation of Development Dependencies

```sh
# Add required Rust toolchain
rustup target install thumbv6m-none-eabi

cargo install flip-link

cargo install elf2uf2-rs --locked
```

## Running

To be able to upload the binary to the Raspberry Pi Pico it will need to
be mounted. This can be achieved by booting the Pico into
"USB Bootloader mode", by rebooting the pico whilst holding the button
labelled "BOOTSEL" on the pico.

Once the Pico has rebooted it can be mounted using the provided script.
NOTE: This script does not work on MacOS instead the Pico is auto mounted.

```sh
./mount-pi.sh
```

Alternatively the raw command can be used

```sh
mkdir -p .rpi-mount
sudo mount /dev/disk/by-label/RPI-RP2 .rpi-mount -o "uid=$UID,gid=users"
```

Once the Rpi Pico has been mounted, simply use

```sh
cargo run
```

To build the binary in development mode and upload it to the Pi Pico.
To build in release mode instead use:

```sh
cargo run --release
```
