use defmt::Format;

#[derive(Clone, Copy, Format)]
pub struct PicoState {
    // Keeps track of whether the remote pico has acknowledged a sent message.
    pico_sent_state: AckState,
    // Keeps track of whether the remote user has acknowledged a sent message.
    user_sent_state: AckState,
    // Keeps track of whether the local pico has acked a recieved message.
    local_recieved_state: AckState,
    // The connection state of the socket
    socket_connected: bool,
}

#[derive(Clone, Copy, Format)]
enum AckState {
    Ack,
    NoAck,
}

impl PicoState {
    pub fn new() -> Self {
        PicoState {
            pico_sent_state: AckState::Ack,
            user_sent_state: AckState::Ack,
            local_recieved_state: AckState::Ack,
            socket_connected: true,
        }
    }

    pub fn send_face(&mut self) {
        self.pico_sent_state = AckState::NoAck;

        self.user_sent_state = AckState::NoAck;
    }

    pub fn recieve_pico_ack(&mut self) {
        self.pico_sent_state = AckState::Ack;
    }

    pub fn recieve_user_ack(&mut self) {
        self.user_sent_state = AckState::Ack;
    }

    pub fn recieved_face(&mut self) {
        self.local_recieved_state = AckState::NoAck;
    }

    pub fn local_acknowledge_recieved(&mut self) {
        self.local_recieved_state = AckState::Ack;
    }

    pub fn local_has_recieved_message(&mut self) -> bool {
        match self.local_recieved_state {
            AckState::Ack => false,
            AckState::NoAck => true,
        }
    }

    pub fn socket_failure(&mut self) {
        self.socket_connected = false;
    }

    pub fn socket_connected(&mut self) {
        self.socket_connected = true;
    }

    pub fn is_socket_connected(&self) -> bool {
        self.socket_connected
    }
}

impl Default for PicoState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Format)]
pub enum ActionRequired {
    None,
    SendAck,
}
