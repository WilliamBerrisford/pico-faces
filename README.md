## pico-faces

![Github Actions Status](https://github.com/WilliamBerrisford/PicoWBirthday/actions/workflows/rust.yml/badge.svg)

Friendly robot(s) built on a Pico-W with Rust & Embassy. Switch between faces and send them to the other bots.

![Two (nearly) finished bots](https://github.com/user-attachments/assets/8d8ba138-76af-46d0-bace-728b282545ff)

### Files for 3D printing
[Available on onshape](https://cad.onshape.com/documents/ab25dae9bb138441e9885842/w/69b030b96d4de9974a289763/e/bb5d3b762e029bd7fe2d0a3f?configuration=List_DaifwpQaD0X30e%3DDefault&renderMode=0&uiState=68865298792d2d6a8b3405fa). These files are under the terms of the Onshape free plan.

The 3d printed parts, the rotary encoder and the breadboard holding the Pi PicoW are secured with M2 threaded heat set inserts and M2 screws. The display is held in place by M2 nuts and M2 screws. 

### Getting started
Setup the hardware, for two bots, you will need:
* 2 x ssd1306 display
* 2 x rotary encoders
* 2 x Pi PicoW

Wired up as follows:

<img width="600" height="762" alt="Screenshot from 2023-11-29 20-42-33" src="https://github.com/user-attachments/assets/836ef9c4-7145-4494-b125-d1ad7cfbbdcb" />

Install probe-rs <https://probe.rs/docs/getting-started/installation/> and attach a debug probe a Pi PicoW (e.g. <https://github.com/raspberrypi/debugprobe> flashed on a Pi Pico).

Create a .env file in the top level of the repo containing the following:

```
WIFI_NETWORK=<SSID1>,<SSID2,<SSIDn>
WIFI_PASSWORD=<password1>,<password2>,<passwordn>

MQTT_PORT=<port>
MQTT_SERVER=<server>

# Credentials, these set the ID's and their corresponding topics
# (used by the MQTT protocol for publishing/subscribing). 

# Test Credentials - these are intended for a test board while the
# real bots use M_ID and W_ID. These correspond to the features
# documented further down.
TEST_ID_ONE=<test-id1>
TEST_ID_TWO=<test-id2>
TEST_TOPIC_ONE=<test1-topic>
TEST_TOPIC_TWO=<test2-topic>

# Real Credentials
M_ID=<id1>
W_ID=<id2>
M_TOPIC=<id1-topic>
W_TOPIC=<id2-topic>
```

Once the debug probe is attached, use `cargo run` with one of the four features `w,m,one,two` (`one` is on by default) to flash the Pico:
```
cargo r -r --features one
```
Each feature corresponds to a user, `one` `two` are able to exchange messages with each other, as are `w` `m`. However, the two groups cannot talk to each other and a user cannot talk to itself. Flashing the 2nd Pi PicoW with the corresponding feature flag will allow them to talk to each other.

### How to use
Rotate the rotary encoder to change faces, press it to send the face to the other bot. The other bot will see "Message Waiting!", press the rotary encoder on that other bot to see the received message. There is one special face; `Sleep Device` which when the rotary encoder is pressed, turns the screen off, to turn the screen back on, simply press the rotatary encoder again.

### Testing
`cargo test` does not work due to only `distance_friend_core` being able to run on x86, instead run tests with:

```
cargo test -p distance_friend_core --target x86_64-unknown-linux-gnu --verbose
```

Thanks to (https://github.com/mdarrik/pico-w-blinky-rust) for an initial working template.
