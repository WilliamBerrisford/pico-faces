use embedded_graphics::{
    Drawable,
    pixelcolor::BinaryColor,
    prelude::{AngleUnit, Point},
    primitives::{Arc, Primitive, PrimitiveStyle},
};
use ssd1306::{
    Ssd1306, mode::BufferedGraphicsMode, prelude::WriteOnlyDataCommand, size::DisplaySize,
};

use super::Eye;

pub struct SemiCircleEye {
    pub base_x: i32,
    pub base_y: i32,
    pub height: u32,
    pub x_offset: i32,
}

impl SemiCircleEye {
    fn single_eye<DI, SIZE>(
        &self,
        display: &mut Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>,
        x: i32,
        height: u32,
    ) where
        DI: WriteOnlyDataCommand,
        SIZE: DisplaySize,
    {
        Arc::new(
            Point::new(x, self.base_y),
            height,
            -180.0.deg(),
            180.0.deg(),
        )
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
        .expect("Failed to draw to display!");
    }
}

impl Eye for SemiCircleEye {
    fn new(base_x: i32, base_y: i32, height: u32, x_offset: i32) -> Self {
        SemiCircleEye {
            base_x,
            base_y,
            height,
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
        self.single_eye(display, self.base_x, self.height);
        self.single_eye(display, self.base_x + self.x_offset, self.height);
    }

    async fn blink<DI, SIZE>(
        &self,
        display: &mut ssd1306::Ssd1306<DI, SIZE, ssd1306::mode::BufferedGraphicsMode<SIZE>>,
        divider: u32,
    ) where
        DI: ssd1306::prelude::WriteOnlyDataCommand,
        SIZE: ssd1306::size::DisplaySize,
    {
        self.single_eye(display, self.base_x, self.height / divider);
        self.single_eye(display, self.base_x + self.x_offset, self.height / divider);
    }
}
