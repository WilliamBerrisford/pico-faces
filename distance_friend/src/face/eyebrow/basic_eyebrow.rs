use embedded_graphics::{
    Drawable,
    pixelcolor::BinaryColor,
    prelude::{AngleUnit, Point},
    primitives::{Arc, Primitive, PrimitiveStyle},
};
use ssd1306::{
    Ssd1306, mode::BufferedGraphicsMode, prelude::WriteOnlyDataCommand, size::DisplaySize,
};

use super::EyeBrow;

pub struct BasicEyebrow {
    base_x: i32,
    base_y: i32,
    x_offset: i32,
}

impl BasicEyebrow {
    fn single_eyebrow<DI, SIZE>(
        display: &mut Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>,
        x: i32,
        height: i32,
    ) where
        DI: WriteOnlyDataCommand,
        SIZE: DisplaySize,
    {
        Arc::new(Point::new(x, height), 32, 225.0.deg(), 90.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(display)
            .expect("Failed to draw to display!");
    }
}

impl EyeBrow for BasicEyebrow {
    fn new(base_x: i32, base_y: i32, x_offset: i32) -> Self {
        BasicEyebrow {
            base_x,
            base_y,
            x_offset,
        }
    }

    async fn normal<DI, SIZE>(
        &self,
        display: &mut ssd1306::Ssd1306<DI, SIZE, ssd1306::mode::BufferedGraphicsMode<SIZE>>,
    ) where
        DI: ssd1306::prelude::WriteOnlyDataCommand,
        SIZE: ssd1306::size::DisplaySize,
    {
        Self::single_eyebrow(display, self.base_x, self.base_y);
        Self::single_eyebrow(display, self.base_x + self.x_offset, self.base_y);
    }
}
