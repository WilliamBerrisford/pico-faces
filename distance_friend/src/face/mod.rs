#[cfg(feature = "embedded")]
mod basic_face;
#[cfg(feature = "embedded")]
mod basic_no_eyebrows;
#[cfg(feature = "embedded")]
mod connecting;
#[cfg(feature = "embedded")]
mod connection_failed;
#[cfg(feature = "embedded")]
mod message_face;

mod eye;
mod eyebrow;
mod mouth;

#[cfg(feature = "embedded")]
mod basic_face_smile;
#[cfg(feature = "embedded")]
mod circle_face;
#[cfg(feature = "embedded")]
mod message_waiting;
#[cfg(feature = "embedded")]
mod semi_circle_face;
#[cfg(feature = "embedded")]
mod sleeping_face;

#[cfg(feature = "embedded")]
pub use crate::face::basic_face::BasicFace;
#[cfg(feature = "embedded")]
pub use crate::face::basic_face_smile::BasicFaceSmile;
#[cfg(feature = "embedded")]
pub use crate::face::basic_no_eyebrows::BasicNoEyebrows;
#[cfg(feature = "embedded")]
pub use crate::face::circle_face::CircleFace;
#[cfg(feature = "embedded")]
pub use crate::face::connecting::Connecting;
#[cfg(feature = "embedded")]
pub use crate::face::connection_failed::ConnectionFailed;
#[cfg(feature = "embedded")]
pub use crate::face::message_face::MessageFace;
#[cfg(feature = "embedded")]
pub use crate::face::message_waiting::MessageWaiting;
#[cfg(feature = "embedded")]
pub use crate::face::semi_circle_face::SemiCircleFace;
#[cfg(feature = "embedded")]
pub use crate::face::sleeping_face::SleepingFace;

pub trait Face {
    fn new() -> Self;

    async fn show<DI, SIZE>(
        &self,
        display: &mut ssd1306::Ssd1306<DI, SIZE, ssd1306::mode::BufferedGraphicsMode<SIZE>>,
    ) where
        DI: ssd1306::prelude::WriteOnlyDataCommand,
        SIZE: ssd1306::size::DisplaySize;

    async fn animate<DI, SIZE>(
        &self,
        display: &mut ssd1306::Ssd1306<DI, SIZE, ssd1306::mode::BufferedGraphicsMode<SIZE>>,
    ) where
        DI: ssd1306::prelude::WriteOnlyDataCommand,
        SIZE: ssd1306::size::DisplaySize;
}
