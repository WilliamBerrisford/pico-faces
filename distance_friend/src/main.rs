#![no_std]
#![no_main]

use cyw43::Control;
use cyw43_pio::PioSpi;
use defmt::{debug, info, unwrap, warn};
use distance_friend::utils::select_face::{Faces, LocalFace, RemoteFace};
use distance_friend::utils::status::{FaceState, PicoState};
use embassy_executor::Spawner;
use embassy_futures::select;

use embassy_net::tcp::TcpSocket;
use embassy_net::{Stack, StackResources};

use embassy_rp::{
    bind_interrupts,
    gpio::{Input, Level, Output},
    i2c::{Config, I2c},
    peripherals::*,
    pio::{InterruptHandler, Pio},
};

use embassy_time::{Duration, Timer};

use distance_friend::utils::{
    display, messages,
    messages::Message,
    mqtt, net,
    re_input::{self, UserInput},
    select_face,
};

use ssd1306::{mode::BufferedGraphicsMode, Ssd1306};

use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<
        'static,
        Output<'static, PIN_23>,
        PioSpi<'static, PIN_25, PIO0, 0, DMA_CH0>,
    >,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<cyw43::NetDriver<'static>>) -> ! {
    stack.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::info!("Hello world");

    let peripherals = embassy_rp::init(Default::default());

    let cs = Output::new(peripherals.PIN_25, Level::High);
    let pwr = Output::new(peripherals.PIN_23, Level::Low);

    let mut pio = Pio::new(peripherals.PIO0, Irqs);
    let dma = peripherals.DMA_CH0;
    let spi: PioSpi<
        '_,
        embassy_rp::peripherals::PIN_25,
        embassy_rp::peripherals::PIO0,
        0,
        embassy_rp::peripherals::DMA_CH0,
    > = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        cs,
        peripherals.PIN_24,
        peripherals.PIN_29,
        dma,
    );

    // Include the WiFi firmware and Country Locale Matrix (CLM) blobs.
    let fw = include_bytes!("../firmware/43439A0.bin");
    let clm = include_bytes!("../firmware/43439A0_clm.bin");

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;

    unwrap!(spawner.spawn(wifi_task(runner)));

    control.init(clm).await;

    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    // // Setup Wifi stuff
    let config = embassy_net::Config::dhcpv4(Default::default());

    // Generate random seed
    let seed = 0x04cf_e99a_d317_3cea;

    static STACK: StaticCell<Stack<cyw43::NetDriver<'static>>> = StaticCell::new();
    static RESOURCES: StaticCell<StackResources<10>> = StaticCell::new();
    let stack = &*STACK.init(Stack::new(
        net_device,
        config,
        RESOURCES.init(StackResources::<10>::new()),
        seed,
    ));

    unwrap!(spawner.spawn(net_task(stack)));

    let mut local_face = LocalFace::new();
    let mut remote_face = RemoteFace::default();

    // Setup i2c display
    let i2c_scl = peripherals.PIN_13;
    let i2c_sda = peripherals.PIN_12;

    let i2c = I2c::new_blocking(peripherals.I2C0, i2c_scl, i2c_sda, Config::default());

    let interface = ssd1306::I2CDisplayInterface::new(i2c);
    let mut display = ssd1306::Ssd1306::new(
        interface,
        ssd1306::size::DisplaySize128x64,
        ssd1306::rotation::DisplayRotation::Rotate0,
    )
    .into_buffered_graphics_mode();

    display::init_display(&mut display).await;

    network_connect(&mut display, &mut control, stack).await;

    let mut tx_buffer = [0u8; 4096];
    let mut rx_buffer = [0u8; 4096];

    let mut mqtt_socket = mqtt::attempt_setup_mqtt(stack, &mut rx_buffer, &mut tx_buffer).await;

    loop {
        match mqtt_socket {
            Some(_) => {
                break;
            }
            None => {
                drop(mqtt_socket);
                mqtt_socket = mqtt::attempt_setup_mqtt(stack, &mut rx_buffer, &mut tx_buffer).await;
            }
        }
    }

    let mut mqtt_socket = mqtt_socket.expect("No mqtt socket set up");

    // Setup rotary encoder pins
    let mut clk = Input::new(peripherals.PIN_4, embassy_rp::gpio::Pull::Up);
    let mut dt = Input::new(peripherals.PIN_5, embassy_rp::gpio::Pull::Up);
    let mut sw = Input::new(peripherals.PIN_6, embassy_rp::gpio::Pull::Up);

    let mut read_buf = [0u8; 1024];
    let serde_buf = [0u8; 32];

    let mut state = PicoState::new();
    let mut led_state = true;

    // Main program loop
    loop {
        debug!("Main Loop");

        if !state.is_socket_connected() {
            warn!("TCP Socket has disconnected, attempting to reconnect.");
            drop(mqtt_socket);
            mqtt_socket = mqtt::attempt_setup_mqtt(stack, &mut rx_buffer, &mut tx_buffer)
                .await
                .expect("Failed to connect to mqtt broker");
            state.socket_connected();
            info!("Socket reconnected sucessfully.");
        }

        control.gpio_set(0, led_state).await;
        led_state = !led_state;

        debug!("Current state: {}", state);

        debug!("Local face: {}", local_face);

        info!("Socket state: {}", mqtt_socket.state());

        let chosen_face = use_state(&mut state, &mut remote_face, &local_face);

        let rotary_input = re_input::input(&mut clk, &mut dt, &mut sw);

        let mqtt_listen = messages::listen(
            &mut read_buf,
            serde_buf,
            &mut mqtt_socket,
            &mut remote_face,
            &mut state,
        );
        let show_face = select_face::show_face(chosen_face, &mut display);
        let loop_result = select::select3(rotary_input, show_face, mqtt_listen).await;

        if let select::Either3::First(user_input) = loop_result {
            on_input(
                user_input,
                &mut local_face,
                &mut mqtt_socket,
                serde_buf,
                &mut state,
            )
            .await;
        }
    }
}

fn use_state(state: &mut PicoState, remote_face: &mut RemoteFace, local_face: &LocalFace) -> Faces {
    if state.sleep_mode {
        debug!("In sleep mode, show blank screen");
        return Faces::SleepingFace;
    }

    if state.local_has_recieved_message() {
        return Faces::MessageWaiting;
    }

    match state.face_state {
        FaceState::Local => {
            info!("Using local face!");
            *local_face.get_face()
        }
        FaceState::Remote => {
            info!("Using remote face!");
            remote_face.get_face()
        }
    }
}

async fn network_connect<DI, SIZE>(
    display: &mut Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>,
    control: &mut Control<'_>,
    stack: &'static Stack<cyw43::NetDriver<'static>>,
) where
    DI: ssd1306::prelude::WriteOnlyDataCommand,
    SIZE: ssd1306::size::DisplaySize,
{
    let connecting_face = select_face::show_face(Faces::Connecting, display);
    let connecting = net::connect_to_network(control, stack);
    if let select::Either::Second(has_connected) = select::select(connecting_face, connecting).await
    {
        match has_connected {
            Ok(_) => (),
            Err(_) => {
                // Show connection failure and will loop indefinitely on
                // connection failure screen.
                select_face::show_face(Faces::ConnectionFailed, display).await;
            }
        };
    } else {
        // show_face loops indefinitely so this will not be reached
        unreachable!();
    }
}

async fn on_input(
    user_input: UserInput,
    local_face: &mut LocalFace,
    mqtt_socket: &mut TcpSocket<'_>,
    serde_buf: [u8; 32],
    state: &mut PicoState,
) {
    match state.sleep_mode {
        true => on_input_asleep(user_input, state).await,
        false => on_input_awake(user_input, local_face, state, mqtt_socket, serde_buf).await,
    }

    // Prevent multiple presses of the button
    if user_input == UserInput::ButtonPress {
        Timer::after(Duration::from_millis(250)).await;
    }
}

async fn on_input_asleep(user_input: UserInput, state: &mut PicoState) {
    if user_input == UserInput::ButtonPress {
        state.sleep_mode = false;
    }
}

async fn on_input_awake(
    user_input: UserInput,
    local_face: &mut LocalFace,
    state: &mut PicoState,
    mqtt_socket: &mut TcpSocket<'_>,
    serde_buf: [u8; 32],
) {
    match user_input {
        UserInput::Clockwise => {
            if state.local_has_acked_message() {
                local_face.next();
                debug!("Clockwise");
                state.face_state = FaceState::Local
            }
        }
        UserInput::AntiClockwise => {
            if state.local_has_acked_message() {
                local_face.prev();
                debug!("Anti-clockwise");
                state.face_state = FaceState::Local
            }
        }
        UserInput::ButtonPress => {
            if state.local_has_recieved_message() {
                info!("Sending user ack");

                messages::send_message(&Message::UserAck, mqtt_socket, serde_buf, state).await;
                state.local_acknowledge_recieved();
            } else {
                info!("Have not recieved message!");
                if state.face_state == FaceState::Remote {
                    state.face_state = FaceState::Local
                } else if *local_face.get_face() == Faces::GoToSleep {
                    state.sleep_mode = true
                } else {
                    info!("Sending face: {}", local_face.get_face());
                    messages::send_message(
                        &Message::ChangeFace(*local_face.get_face()),
                        mqtt_socket,
                        serde_buf,
                        state,
                    )
                    .await;
                    state.send_face();
                }
            }
        }
    }
}
