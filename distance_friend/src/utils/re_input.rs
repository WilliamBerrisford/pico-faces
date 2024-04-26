use defmt::debug;
use embassy_futures::select;
use embassy_rp::{
    gpio::Input,
    peripherals::{PIN_4, PIN_5, PIN_6},
};

use distance_friend_core::external::encoder::{EncoderDirection, MetaEncoderState};

#[derive(Clone, Copy)]
pub enum UserInput {
    Clockwise,
    AntiClockwise,
    ButtonPress,
}

pub async fn input(
    clk: &mut Input<'_, PIN_4>,
    dt: &mut Input<'_, PIN_5>,
    sw: &mut Input<'_, PIN_6>,
) -> UserInput {
    let mut state = MetaEncoderState::new();

    loop {
        debug!("input");
        let user_input = select::select3(
            clk.wait_for_any_edge(),
            dt.wait_for_any_edge(),
            sw.wait_for_low(),
        )
        .await;

        let clk_state = clk.is_high();
        let dt_state = dt.is_low();

        if let select::Either3::Third(_) = user_input {
            return UserInput::ButtonPress;
        }

        state.next(clk_state, dt_state);
        let direction = state.get_direction();

        match direction {
            EncoderDirection::Clockwise => {
                return UserInput::Clockwise;
            }
            EncoderDirection::AntiClockwise => {
                return UserInput::AntiClockwise;
            }
            EncoderDirection::Bounce => (),
        }

        state.update_last();
    }
}
