use defmt::Format;
use serde::{Deserialize, Serialize};
use ssd1306::{
    mode::BufferedGraphicsMode, prelude::WriteOnlyDataCommand, size::DisplaySize, Ssd1306,
};

use crate::face::{
    BasicFace, BasicFaceSmile, BasicNoEyebrows, CircleFace, Connecting, ConnectionFailed, Face,
    MessageFace, MessageWaiting, SemiCircleFace, SleepingFace,
};

pub const NUM_FACES: usize = 9;

#[derive(Clone, Copy, Serialize, Deserialize, Format, PartialEq)]
pub enum Faces {
    Basic,
    BasicNoEyebrows,
    SemiCircleFace,
    CircleFace,
    BasicSmile,
    GoToSleep,
    // Message faces:
    Hello,
    GoodMorning,
    GoodNight,
    // Special Case Fases
    MessageWaiting,
    Connecting,
    ConnectionFailed,
    SleepingFace,
}

impl Default for Faces {
    fn default() -> Self {
        Self::Basic
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Format, Default)]
pub struct RemoteFace {
    pub(crate) face: Faces,
}

impl RemoteFace {
    pub fn set_face(&mut self, chosen_face: Faces) {
        self.face = chosen_face;
    }

    pub fn get_face(&mut self) -> Faces {
        self.face
    }
}

impl Default for LocalFace {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Format)]
pub struct LocalFace {
    pub(crate) faces: [Faces; NUM_FACES],
    pub(crate) current_index: u32,
}

impl LocalFace {
    pub fn new() -> LocalFace {
        LocalFace {
            faces: [
                Faces::Basic,
                Faces::BasicNoEyebrows,
                Faces::SemiCircleFace,
                Faces::CircleFace,
                Faces::BasicSmile,
                Faces::Hello,
                Faces::GoodMorning,
                Faces::GoodNight,
                Faces::GoToSleep,
            ],
            current_index: 0,
        }
    }

    pub fn next(&mut self) {
        if self.current_index + 1
            >= u32::try_from(NUM_FACES).expect("Number of faces should be convertable to u32")
        {
            self.current_index = 0;
        } else {
            self.current_index += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.current_index == 0 {
            self.current_index = (NUM_FACES - 1)
                .try_into()
                .expect("Number of faces should be convertable to u32")
        } else {
            self.current_index -= 1;
        }
    }

    pub fn get_face(&self) -> &Faces {
        return self
            .faces
            .get(
                usize::try_from(self.current_index)
                    .expect("current_index not convertable to usize"),
            )
            .expect("Face must exist at index");
    }
}

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
        Faces::SleepingFace => SleepingFace::new().show(display).await,
        Faces::GoToSleep => {
            MessageFace::new_with_message("Sleep Device")
                .show(display)
                .await
        }
    }
}
