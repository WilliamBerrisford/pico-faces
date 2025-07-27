## Pico-W-Birthday

![Github Actions Status](https://github.com/WilliamBerrisford/PicoWBirthday/actions/workflows/rust.yml/badge.svg)

A birthday present on a Pico-W with Rust & Embassy. 

### Testing
`cargo test` does not work due to only distance_friend_core being abled to run on x86, instead run tests with:

```
cargo test -p distance_friend_core --target x86_64-unknown-linux-gnu --verbose
```

Thanks to (https://github.com/mdarrik/pico-w-blinky-rust) for an initial working template.
