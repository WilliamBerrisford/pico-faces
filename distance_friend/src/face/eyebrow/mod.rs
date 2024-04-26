use ssd1306::{
    mode::BufferedGraphicsMode, prelude::WriteOnlyDataCommand, size::DisplaySize, Ssd1306,
};

mod basic_eyebrow;

pub use basic_eyebrow::BasicEyebrow;

pub trait EyeBrow {
    fn new(base_x: i32, base_y: i32, x_offset: i32) -> Self;

    async fn normal<DI, SIZE>(&self, display: &mut Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>)
    where
        DI: WriteOnlyDataCommand,
        SIZE: DisplaySize;
}
