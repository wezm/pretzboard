Pretzboard
==========

The aim of this project is to... These things might include:

* Time & date
* Current weather

Building & Running
------------------

The project is implemented in [Rust] and uses the [Piston] framework. Rust
version 1.28.0 or newer is required to compile the application.

[Rust]: http://rust-lang.org/
[Piston]: http://piston.rs/

Build:

    cargo build --release

Run:

    cargo run --release

Raspberry Pi
------------

Cross-compling makes use of the cross tool and Docker images.

<https://github.com/wezm/cross/tree/wallflower>

Build:

    cross build --target=armv7-unknown-linux-gnueabihf --release

Run:

    SSL_CERT_DIR=/etc/ssl/certs ./pretzboard

