# 🏓 Pong

Built in Rust following [Amethyst.rs' pong tutorial](https://book.amethyst.rs/book/stable/pong-tutorial).

## Running
`cargo run`

## Building
`cargo build`

## Caviats

* I ran into `[BUG] attempted to leave type platform::platform::x11::util::input::PointerState uninitialized`, which required me to run `rustup override set 1.47.0` in project directory. [Credit](https://github.com/amethyst/amethyst/issues/2524#issuecomment-817255546).

## Motivation
* Research and apply Entity Component System architecture.
* Learn and apply some Rust.
* Build some more game-dev skills.
* See how I like Amethyst.

## Goals
* [ ] Build pong following tutorial.
* [ ] Write tests.
* [ ] Expand a feature or two into the game for practice.
* [ ] *Optional:* Build to wasm for blog.
