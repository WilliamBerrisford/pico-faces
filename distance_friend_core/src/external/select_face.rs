use defmt::Format;
use serde::{Deserialize, Serialize};

pub const NUM_FACES: usize = 9;

#[derive(Clone, Copy, Serialize, Deserialize, Format, PartialEq)]
pub enum Faces {
    Basic,
    BasicNoEyebrows,
    SemiCircleFace,
    CircleFace,
    BasicSmile,
    MessageWaiting,
    Connecting,
    ConnectionFailed,
    // Message faces:
    Hello,
    GoodMorning,
    GoodNight,
    SleepingFace,
    GoToSleep,
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

#[derive(Serialize, Deserialize, Format)]
pub struct LocalFace {
    pub(crate) faces: [Faces; NUM_FACES],
    pub(crate) current_index: u32,
}

impl Default for LocalFace {
    fn default() -> Self {
        Self::new()
    }
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
