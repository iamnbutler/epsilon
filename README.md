A work-in-progress personal life manager & assistant written in Rust. It is meant to be deployed to an environment like a rpi, on a device like the ClockworkPi.

GUI will be built with GPUI

## Setup

2. Install the `sqlx-cli` via `cargo install sqlx-cli`.
3. Rename `.env.example` to `.env` and update values

## Running

1. Run `script/db` to set up db and run migrations.
2. Run `cargo run`.
