use core::str::from_utf8;

use defmt::{info, Format};
use serde::{Deserialize, Serialize};

use super::{
    select_face::{Faces, RemoteFace},
    status::{ActionRequired, PicoState},
};

#[derive(Clone, Copy, Serialize, Deserialize, Format)]
pub enum Message {
    PicoAck,
    UserAck,
    ChangeFace(Faces),
}

pub fn process_message(
    publish: mqttrs::Publish<'_>,
    state: &mut PicoState,
    remote_face: &mut RemoteFace,
) -> ActionRequired {
    let recieved = postcard::from_bytes::<Message>(publish.payload);

    match recieved {
        Ok(recieved_face) => match recieved_face {
            Message::PicoAck => {
                info!("Pico Ack recieved!");
                state.recieve_pico_ack();
            }
            Message::ChangeFace(recieved_face) => {
                info!("Face state recieved: {}", recieved_face);
                remote_face.set_face(recieved_face);
                state.recieved_face();
                return ActionRequired::SendAck;
            }
            Message::UserAck => {
                info!("User Ack recieved");
                state.recieve_user_ack();
            }
        },
        Err(_) => {
            info!(
                "Not face state, payload as str: {}",
                from_utf8(publish.payload).unwrap_or("Could not decode payload to str")
            );
        }
    }
    ActionRequired::None
}
