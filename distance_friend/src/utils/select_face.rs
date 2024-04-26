use ssd1306::{
    mode::BufferedGraphicsMode, prelude::WriteOnlyDataCommand, size::DisplaySize, Ssd1306,
};

use crate::face::{
    BasicFace, BasicFaceSmile, BasicNoEyebrows, CircleFace, Connecting, ConnectionFailed, Face,
    MessageFace, MessageWaiting, SemiCircleFace,
};

use distance_friend_core::external::select_face::Faces;

pub async fn show_face<DI, SIZE>(
    chosen_face: Faces,
    display: &mut Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>,
) where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    match chosen_face {
        Faces::Basic => BasicFace::new().show(display).await,
        Faces::BasicNoEyebrows => BasicNoEyebrows::new().show(display).await,
        Faces::SemiCircleFace => SemiCircleFace::new().show(display).await,
        Faces::MessageWaiting => MessageWaiting::new().show(display).await,
        Faces::Connecting => Connecting::new().show(display).await,
        Faces::ConnectionFailed => ConnectionFailed::new().show(display).await,
        Faces::Hello => MessageFace::new_with_message("Hello!").show(display).await,
        Faces::GoodMorning => {
            MessageFace::new_with_message("Good\nMorning!")
                .show(display)
                .await
        }
        Faces::GoodNight => {
            MessageFace::new_with_message("Good\nNight!")
                .show(display)
                .await
        }
        Faces::CircleFace => CircleFace::new().show(display).await,
        Faces::BasicSmile => BasicFaceSmile::new().show(display).await,
    }
}
