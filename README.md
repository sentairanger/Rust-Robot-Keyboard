# Rust-Robot-Keyboard
This project uses Rust to control a Robot via keyboard control

## Introduction

This project was a challenge that was sent to me and here it is. This uses Rust to control a robot. To run the application first you must have rust installed on your Pi. This [link](https://doc.rust-lang.org/book/ch01-01-installation.html) shows you how to install Rust on your Pi. This will work on any Pi, including the Pi 5.The `Cargo.toml` file contains the needed dependencies `termion` for keyboard input and `rppal` for Raspberry Pi access. To run first run `cargo build` then `cargo run`. Press the keys and the robot should move. Press Enter to stop the robot. Press Q to quit the application.

## Requirements

* Raspberry Pi (any Pi will suffice but a Pi with WiFi and Bluetooth is recommended)
* Raspberry Pi OS Bullseye and newer
* Rust
* A keyboard whether it's USB or Bluetooth
