## Pico-W-Birthday

![Github Actions Status](https://github.com/WilliamBerrisford/PicoWBirthday/actions/workflows/rust.yml/badge.svg)

A birthday present on a Pico-W with Rust & Embassy. Based on the (https://github.com/mdarrik/pico-w-blinky-rust).

### Testing
`cargo test` does not work due to workspace issues, instead run tests with:

```
cargo test -p distance_friend_core --target x86_64-unknown-linux-gnu --verbose
```

TODO Write proper README
