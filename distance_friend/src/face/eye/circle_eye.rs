use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::Point,
    primitives::{Circle, Primitive, PrimitiveStyleBuilder},
    Drawable,
};
use ssd1306::{
    mode::BufferedGraphicsMode, prelude::WriteOnlyDataCommand, size::DisplaySize, Ssd1306,
};

use super::Eye;

pub struct CircleEye {
    pub base_x: i32,
    pub base_y: i32,
    pub height: u32,
    pub x_offset: i32,
}

impl CircleEye {
    fn single_eye<DI, SIZE>(
        &self,
        display: &mut Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>,
        x: i32,
        height: u32,
    ) where
        DI: WriteOnlyDataCommand,
        SIZE: DisplaySize,
    {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .fill_color(BinaryColor::On)
            .stroke_width(1)
            .build();

        Circle::new(
            Point::new(
                x,
                self.base_y - i32::try_from(height).expect("Height must be convertable to i32"),
            ),
            height,
        )
        .into_styled(style)
        .draw(display)
        .expect("Failed to draw to display!");
    }
}

impl Eye for CircleEye {
    fn new(base_x: i32, base_y: i32, height: u32, x_offset: i32) -> Self {
        CircleEye {
            base_x,
            base_y,
            height,
            x_offset,
        }
    }

    async fn normal<DI, SIZE>(&self, display: &mut Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>)
    where
        DI: WriteOnlyDataCommand,
        SIZE: DisplaySize,
    {
        self.single_eye(display, self.base_x, self.height);
        self.single_eye(display, self.base_x + self.x_offset, self.height);
    }

    async fn blink<DI, SIZE>(
        &self,
        display: &mut Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>,
        divider: u32,
    ) where
        DI: WriteOnlyDataCommand,
        SIZE: DisplaySize,
    {
        self.single_eye(display, self.base_x, self.height / divider);
        self.single_eye(display, self.base_x + self.x_offset, self.height / divider);
    }
}
