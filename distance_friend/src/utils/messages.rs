use defmt::{dbg, debug, error, info, warn};
use embassy_net::tcp::TcpSocket;
use embassy_time::{Duration, Timer};
use mqttrs::Packet;

use crate::utils::mqtt;

use distance_friend_core::external::{
    messages,
    messages::Message,
    select_face::RemoteFace,
    status::{ActionRequired, PicoState},
};

const INVALID_LIMIT: u32 = 10;

pub async fn send_message(
    message: &Message,
    mqtt_socket: &mut TcpSocket<'_>,
    mut serde_buf: [u8; 32],
    state: &mut PicoState,
) {
    debug!("Socket state: {}", mqtt_socket.state());
    match mqtt::publish_state(
        mqtt_socket,
        postcard::to_slice(message, &mut serde_buf).expect("Failed to serialise local face"),
    )
    .await
    {
        Ok(_) => {
            info!("Successfully published message: {}", message);
        }
        Err(e) => {
            error!("Failed to publish state! {}", e);
            state.socket_failure();
        }
    }
}

pub async fn listen<'a>(
    read_buf: &'a mut [u8; 1024],
    serde_buf: [u8; 32],
    socket: &mut TcpSocket<'_>,
    remote_face: &mut RemoteFace,
    state: &mut PicoState,
) {
    let mut invalid_count: u32 = 0;
    loop {
        match mqtt::listen(read_buf, socket).await {
            Some(Packet::Publish(publish)) => {
                info!("Valid packet recieved, Topic name: {}", publish.topic_name);
                if let ActionRequired::SendAck =
                    messages::process_message(publish, state, remote_face)
                {
                    let _ = send_message(&Message::PicoAck, socket, serde_buf, state);
                    return;
                }
            }
            Some(p) => {
                dbg!("Other packet recieved ignoring {:#?}", p.get_type());
            }
            None => {
                if invalid_count > INVALID_LIMIT {
                    error!("Exceeded invalid packet limit while listening",);
                    state.socket_failure();
                    return;
                }

                warn!("Invalid packet recieved");
                invalid_count += 1;
                Timer::after(Duration::from_secs(10)).await;
            }
        }
    }
}
