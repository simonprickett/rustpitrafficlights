# Experiments with Low Voltage Labs Traffic Lights on Raspberry Pi with Rust

Let's look at how to control the Raspberry Pi's GPIO pins using the Rust programming language, and the [Low Voltage Labs traffic light LEDs](https://lowvoltagelabs.com/products/pi-traffic/).

For a complete description of how this code works, and a shopping list of things you'll need to make it run on a Raspberry Pi, check out the article on my website... TODO LINK TO FULL ARTICLE

We'll also use the [`rust_gpiozero` crate](https://crates.io/crates/rust_gpiozero) to control the state of the GPIO pins.

## Attaching the Traffic Lights

Attach the traffic lights so that they are connected to GPIO 9, 10, 11 ([Broadcom/BCM pin numbering](https://pinout.xyz/)).

## Install Rust

Installing Rust also installs [Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html), a build system and package manager for Rust projects.

Follow the installation process [here](https://www.rust-lang.org/tools/install), which at the time of writing looks like this:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Clone, Compile and Run the Code

First, clone this repository:

```bash
$ git clone https://github.com/simonprickett/rustpitrafficlights.git
```

Then `cd` into the `rustpitrafficlights` folder and run the code:

```bash
$ cd rustpitrafficlights
$ cargo run
```

If all is well and the traffic lights are connected correctly, they should start to display the UK traffic light sequence (red, red & yellow, green, yellow, red).  Exit the program by pressing `Ctrl+C`.

If you don't see any lights, make sure that your traffic lights are connected to the expected GPIO pins.
