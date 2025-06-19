use embassy_time::{Duration, Timer};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::BinaryColor,
    prelude::{Dimensions, Point},
    text::{Alignment, Text},
};

use super::Face;

const DELAY_SECS: u64 = 2;

pub struct Connecting {}

impl Face for Connecting {
    fn new() -> Self {
        Connecting {}
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
            y_pos += 2;

            if y_pos > 25 {
                y_pos = 0;
            }

            top = !top;

            let _ = display.clear(BinaryColor::Off);
            let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

            let text = "Connecting\n to WiFi";
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
