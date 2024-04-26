use defmt::{debug, error, info};
use dotenvy_macro::dotenv;
use embassy_net::{
    dns::{DnsQueryType, DnsSocket},
    tcp::{Error, TcpSocket},
    Stack,
};
use embassy_time::Duration;
use embassy_usb::class::cdc_ncm::embassy_net::Device;
use heapless::Vec;
use mqttrs::{Connect, Packet, Pid, Protocol, Subscribe, SubscribeTopic};

const KEEP_ALIVE_TIME: u32 = 120;

pub async fn attempt_setup_mqtt<'a: 'b, 'b>(
    stack: &'static Stack<Device<'_, 1514>>,
    rx_buffer: &'a mut [u8],
    tx_buffer: &'a mut [u8],
) -> Option<TcpSocket<'b>> {
    let mut socket = connect_to_broker(stack, rx_buffer, tx_buffer).await?;
    subscribe(&mut socket).await.ok()?;
    info!("MQTT Setup");
    Some(socket)
}

async fn connect_to_broker<'a>(
    stack: &'static Stack<Device<'_, 1514>>,
    rx_buffer: &'a mut [u8],
    tx_buffer: &'a mut [u8],
) -> Option<TcpSocket<'a>> {
    let mut socket = TcpSocket::new(stack, rx_buffer, tx_buffer);

    socket.set_keep_alive(Some(Duration::from_secs((KEEP_ALIVE_TIME).into())));
    socket.set_timeout(Some(Duration::from_secs((KEEP_ALIVE_TIME * 2).into())));

    let dns_socket = DnsSocket::new(stack);
    info!("Querying dns");
    let mqtt_server_ipv4_address = dns_socket
        .query(dotenv!("MQTT_SERVER"), DnsQueryType::A)
        .await
        .expect("Failed to find dns record")[0];

    info!(
        "connecting to {:?}:{}...",
        mqtt_server_ipv4_address,
        dotenv!("MQTT_PORT")
    );
    if let Err(e) = socket
        .connect((
            mqtt_server_ipv4_address,
            str::parse::<u16>(dotenv!("MQTT_PORT")).unwrap(),
        ))
        .await
    {
        error!("connect error: {:?}", e);
        return None;
    }
    info!("connected to broker");
    send_connect(&mut socket).await.ok()?;
    Some(socket)
}

pub async fn send_connect(socket: &mut TcpSocket<'_>) -> Result<(), Error> {
    #[cfg(feature = "one")]
    let id = dotenv!("TEST_ID_ONE");

    #[cfg(feature = "two")]
    let id = dotenv!("TEST_ID_TWO");

    #[cfg(feature = "m")]
    let id = dotenv!("M_ID");

    #[cfg(feature = "w")]
    let id = dotenv!("W_ID");

    info!("Client ID: {}", id);

    let packet = Packet::Connect(Connect {
        protocol: Protocol::MQTT311,
        keep_alive: u16::MAX - 1,
        client_id: id,
        clean_session: true,
        last_will: None,
        username: None,
        password: None,
    });

    send_packet(&packet, socket).await
}

pub async fn publish_state(socket: &mut TcpSocket<'_>, content: &[u8]) -> Result<(), Error> {
    #[cfg(feature = "one")]
    let topic = dotenv!("TEST_TOPIC_ONE");

    #[cfg(feature = "two")]
    let topic = dotenv!("TEST_TOPIC_TWO");

    #[cfg(feature = "m")]
    let topic = dotenv!("M_TOPIC");

    #[cfg(feature = "w")]
    let topic = dotenv!("W_TOPIC");

    info!("Publishing to {}", topic);
    let packet = Packet::Publish(mqttrs::Publish {
        dup: false,
        qospid: mqttrs::QosPid::AtMostOnce,
        retain: false,
        topic_name: topic,
        payload: content,
    });

    send_packet(&packet, socket).await
}

pub async fn subscribe(socket: &mut TcpSocket<'_>) -> Result<(), Error> {
    let mut topics: Vec<SubscribeTopic, 5> = Vec::new();

    #[cfg(feature = "one")]
    let topic = dotenv!("TEST_TOPIC_TWO");

    #[cfg(feature = "two")]
    let topic = dotenv!("TEST_TOPIC_ONE");

    #[cfg(feature = "m")]
    let topic = dotenv!("W_TOPIC");

    #[cfg(feature = "w")]
    let topic = dotenv!("M_TOPIC");

    info!("Subscribing to {}", topic);
    let test_topic = SubscribeTopic {
        topic_path: topic.into(),
        qos: mqttrs::QoS::AtMostOnce,
    };

    let _ = topics.push(test_topic);

    let packet = Packet::Subscribe(Subscribe {
        pid: Pid::try_from(1).expect("Failed to convert 1 into pid"),
        topics,
    });

    send_packet(&packet, socket).await
}

pub async fn listen<'a>(
    read_buf: &'a mut [u8; 1024],
    socket: &mut TcpSocket<'_>,
) -> Option<Packet<'a>> {
    match socket.read(read_buf).await {
        Ok(read_len) => mqttrs::decode_slice(&read_buf[..read_len]).ok()?,
        Err(e) => {
            debug!("Error reading from socket: {}", e);

            None
        }
    }
}

async fn send_packet(packet: &Packet<'_>, socket: &mut TcpSocket<'_>) -> Result<(), Error> {
    let mut buf = [0u8; 1024];

    let len = mqttrs::encode_slice(packet, &mut buf).expect("Failed to encode slice");

    socket.write(&buf[..len]).await?;
    socket.flush().await
}
