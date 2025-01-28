# Ctrl-C Handling Rust Application

This Rust program demonstrates how to handle Ctrl-C (Control + C) key press events using the `crossterm` crate for terminal manipulation and event handling. The program runs a loop that waits for the Ctrl-C key press and exits gracefully when it is detected.

## Prerequisites

- Rust: Ensure you have Rust installed on your system. If not, follow the instructions below to install it.

## Installation

### Installing Rust

To install Rust, you can use `rustup`, the Rust toolchain installer. Open a terminal and run the following command:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions to complete the installation. Once installed, ensure that the `cargo` command is available in your terminal by running:

```sh
cargo --version
```

### Cloning the Repository

Clone this repository to your local machine using the following command:

```sh
git clone https://github.com/plecos/ctrlc-crossterm.git
cd ctrlc-crossterm
```

## Building the Application

To compile the Rust application, navigate to the project directory and run:

```sh
cargo build 
```

## Running the Application

After building the application, you can run it using the following command:

```sh
cargo run 
```

The program will print "Waiting for Ctrl-C..." to the terminal and will exit with "Got it! Exiting..." when the Ctrl-C key press is detected.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
```
