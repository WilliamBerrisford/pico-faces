use embedded_graphics::{
    Drawable,
    geometry::Point,
    pixelcolor::BinaryColor,
    prelude::AngleUnit,
    primitives::{Arc, Primitive, PrimitiveStyle},
};

use super::Mouth;

pub struct Smile {
    base_x: i32,
    base_y: i32,
}

impl Mouth for Smile {
    fn new(base_x: i32, base_y: i32) -> Self {
        Smile { base_x, base_y }
    }

    async fn normal<DI, SIZE>(
        &self,
        display: &mut ssd1306::Ssd1306<DI, SIZE, ssd1306::mode::BufferedGraphicsMode<SIZE>>,
    ) where
        DI: ssd1306::prelude::WriteOnlyDataCommand,
        SIZE: ssd1306::prelude::DisplaySize,
    {
        Arc::new(
            Point::new(self.base_x, self.base_y),
            40,
            -225.0.deg(),
            -90.0.deg(),
        )
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
        .expect("Failed to draw to display!");
    }
}
