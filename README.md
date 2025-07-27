## Pico-W-Birthday

![Github Actions Status](https://github.com/WilliamBerrisford/PicoWBirthday/actions/workflows/rust.yml/badge.svg)

A friendly robot built on a Pico-W with Rust & Embassy. 

### Files for 3D printing
[Available on onshape](https://cad.onshape.com/documents/ab25dae9bb138441e9885842/w/69b030b96d4de9974a289763/e/bb5d3b762e029bd7fe2d0a3f?configuration=List_DaifwpQaD0X30e%3DDefault&renderMode=0&uiState=68865298792d2d6a8b3405fa). These files are under the terms of the Onshape free plan.

### Testing
`cargo test` does not work due to only distance_friend_core being abled to run on x86, instead run tests with:

```
cargo test -p distance_friend_core --target x86_64-unknown-linux-gnu --verbose
```

Thanks to (https://github.com/mdarrik/pico-w-blinky-rust) for an initial working template.
