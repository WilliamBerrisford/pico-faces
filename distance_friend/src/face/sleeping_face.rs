use embassy_time::{Duration, Timer};
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};

use super::Face;

pub struct SleepingFace {}

const DELAY_SECS: u64 = 60;

impl Face for SleepingFace {
    fn new() -> Self {
        SleepingFace {}
    }

    async fn show<DI, SIZE>(
        &self,
        display: &mut ssd1306::Ssd1306<DI, SIZE, ssd1306::mode::BufferedGraphicsMode<SIZE>>,
    ) where
        DI: ssd1306::prelude::WriteOnlyDataCommand,
        SIZE: ssd1306::size::DisplaySize,
    {
        loop {
            let _ = display.clear(BinaryColor::Off);
            display.flush().expect("Failed to flush display!");
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
