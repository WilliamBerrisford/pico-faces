use embassy_time::{Duration, Timer};
use embedded_graphics::{
    mono_font::{ascii::FONT_5X7, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::{Dimensions, Point},
    text::{Alignment, Text},
    Drawable,
};

use super::Face;

const DELAY_SECS: u64 = 2;

pub struct ConnectionFailed {}

impl Face for ConnectionFailed {
    fn new() -> Self {
        ConnectionFailed {}
    }

    async fn show<DI, SIZE>(
        &self,
        display: &mut ssd1306::Ssd1306<DI, SIZE, ssd1306::mode::BufferedGraphicsMode<SIZE>>,
    ) where
        DI: ssd1306::prelude::WriteOnlyDataCommand,
        SIZE: ssd1306::size::DisplaySize,
    {
        let mut top = true;
        let mut y_pos: i32 = -10;

        loop {
            y_pos += 2;

            if y_pos > 15 {
                y_pos = 0;
            }

            top = !top;

            display.clear();
            let style = MonoTextStyle::new(&FONT_5X7, BinaryColor::On);

            let text = "WiFi Connection Failed\n Restart when known\n network is in range";
            Text::with_alignment(
                text,
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
