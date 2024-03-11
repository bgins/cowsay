# cowsay

Cowsay implemented as a Wasm component.

```
 _____________________
/ This is the best, I \
\ love being a cow!   /
 ---------------------
       \   ^__^
        \  (oo)\_______
           (__)\       )\/\
               ||----w |
               ||     ||
```

This project was adapted from [rsay][rust-rsay].

## Setup

Install the latest stable version of [Rust][install-rust].

Install [`cargo-component`][cargo-component]:

```sh
cargo install cargo-component
```

Add the `wasm32-wasi` target.

```sh
rustup target add wasm32-wasi
```

## Build

Build for the debug target:

```sh
cargo component build
```

Build for release:

```sh
cargo component build --release
```

The builds target `wasm32-wasi` and are compiled to `target/wasm32-wasi/debug/cowsay.wasm` and `target/wasm32-wasi/release/cowsay.wasm` respectively.

## Use

The Wasm component has a [WIT interface][wit-interface] that accepts a message for the Cow to say and a width to determine where to wrap lines.

See the [`fortune` example][fortune-example] for one possible use case.

[cargo-component]: https://github.com/bytecodealliance/cargo-component
[fortune-example]: ./examples/fortune
[install-rust]: https://www.rust-lang.org/tools/install
[rust-rsay]: https://github.com/NickTomlin/rust-rsay
[wit-interface]: ./wit/world.wit
