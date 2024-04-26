use embassy_time::{Duration, Timer};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::{Dimensions, Point},
    text::{Alignment, Text},
    Drawable,
};

use super::Face;

const DELAY_SECS: u64 = 2;

pub struct MessageFace<'a> {
    message: &'a str,
}

impl<'a> MessageFace<'a> {
    pub fn new_with_message(test: &'a str) -> MessageFace<'a> {
        MessageFace { message: test }
    }
}

impl<'a> Face for MessageFace<'a> {
    fn new() -> Self {
        MessageFace { message: "Hello!" }
    }

    async fn show<DI, SIZE>(
        &self,
        display: &mut ssd1306::Ssd1306<DI, SIZE, ssd1306::mode::BufferedGraphicsMode<SIZE>>,
    ) where
        DI: ssd1306::prelude::WriteOnlyDataCommand,
        SIZE: ssd1306::size::DisplaySize,
    {
        let mut top = true;
        let mut y_pos: i32 = 0;

        loop {
            y_pos += 5;

            if y_pos > 15 {
                y_pos = 0;
            }

            top = !top;

            display.clear();
            let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

            Text::with_alignment(
                self.message,
                display.bounding_box().center() + Point::new(0, y_pos),
                style,
                Alignment::Center,
            )
            .draw(display)
            .expect("Failed to draw to display!");

            display.flush().expect("Display failed to flush!");
            Timer::after(Duration::from_secs(DELAY_SECS)).await;
        }
    }

    async fn animate<DI, SIZE>(
        &self,
        _display: &mut ssd1306::Ssd1306<DI, SIZE, ssd1306::mode::BufferedGraphicsMode<SIZE>>,
    ) where
        DI: ssd1306::prelude::WriteOnlyDataCommand,
        SIZE: ssd1306::size::DisplaySize,
    {
    }
}
