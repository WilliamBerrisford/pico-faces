use ssd1306::{
    mode::BufferedGraphicsMode, prelude::WriteOnlyDataCommand, size::DisplaySize, Ssd1306,
};

mod basic_eye;
mod circle_eye;
mod semi_circle_eye;

pub use crate::face::eye::basic_eye::BasicEye;
pub use crate::face::eye::circle_eye::CircleEye;
pub use crate::face::eye::semi_circle_eye::SemiCircleEye;

pub trait Eye {
    fn new(base_x: i32, base_y: i32, height: u32, x_offset: i32) -> Self;

    async fn normal<DI, SIZE>(&self, display: &mut Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>)
    where
        DI: WriteOnlyDataCommand,
        SIZE: DisplaySize;

    async fn blink<DI, SIZE>(
        &self,
        display: &mut Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>,
        divider: u32,
    ) where
        DI: WriteOnlyDataCommand,
        SIZE: DisplaySize;
}
