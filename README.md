# bevy_ecs_tilemap_autotile_example

This project aims to be an exploration of performing AutoTiling on top of the `bevy_ecs_tilemap` crate as part of the
following [RFC](https://github.com/StarArawn/bevy_ecs_tilemap/discussions/326) to add AutoTiling to `bevy_ecs_tilemap`.

# Controls

WASD to move camera.

Scroll in and out to zoom camera.

Click to place tiles.

Press 0 to select blank tiles.
Press 1 to select grass tiles.
Press 2 to select dirt tiles.
Press 3 to select water tiles.

# Assets

I am using the following free assets, please note their READMES and LICENSES.

- https://cupnooble.itch.io/sprout-lands-asset-pack

# Tooling

## clippy

A collection of lints to catch common mistakes and improve your Rust code.

To see suggestions: `cargo clippy`

To automatically apply suggestions: `cargo clippy --fix`

1. https://github.com/rust-lang/rust-clippy

## rustfmt

A tool for formatting Rust code according to style guidelines.

1. https://github.com/rust-lang/rustfmt
2. https://github.com/rust-lang/rustfmt/blob/master/intellij.md (For use with CLion's Rust Plugin)

# Dependencies

See `cargo.toml` for details.

## bevy

Bevy is a refreshingly simple data-driven game engine built in Rust. It is free and open-source forever!

1. https://bevyengine.org/
2. https://bevyengine.org/learn/
3. https://github.com/bevyengine/bevy
4. https://crates.io/crates/bevy

## bevy_ecs_tilemap

A tilemap rendering plugin for bevy. It is more ECS friendly as it makes tiles entities.

1. https://github.com/StarArawn/bevy_ecs_tilemap
2. https://github.com/StarArawn/bevy_ecs_tilemap/tree/main/examples
3. https://docs.rs/bevy_ecs_tilemap/latest/bevy_ecs_tilemap/
4. https://crates.io/crates/bevy_ecs_tilemap

## bevy-inspector-egui

This crate provides a debug interface using egui where you can visually edit the values of your components live.

1. https://github.com/jakobhellermann/bevy-inspector-egui
2. https://crates.io/crates/bevy-inspector-egui
