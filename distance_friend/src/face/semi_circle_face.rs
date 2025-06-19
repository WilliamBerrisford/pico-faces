use embassy_time::{Duration, Timer};
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};

use crate::face::eye::{Eye, SemiCircleEye};

use super::Face;

pub struct SemiCircleFace {
    eyes: SemiCircleEye,
    delay_secs: u64,
}

const EYE_BASE_X: i32 = 20;
const EYE_BASE_Y: i32 = 16;
const EYE_HEIGHT: u32 = 32;
const EYE_X_OFFSET: i32 = 62;

impl Face for SemiCircleFace {
    fn new() -> Self {
        SemiCircleFace {
            eyes: SemiCircleEye::new(EYE_BASE_X, EYE_BASE_Y, EYE_HEIGHT, EYE_X_OFFSET),
            delay_secs: 6,
        }
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
            self.eyes.normal(display).await;
            display.flush().expect("Failed to flush display!");
            Timer::after(Duration::from_secs(self.delay_secs)).await;
            self.animate(display).await;
        }
    }

    async fn animate<DI, SIZE>(
        &self,
        display: &mut ssd1306::Ssd1306<DI, SIZE, ssd1306::mode::BufferedGraphicsMode<SIZE>>,
    ) where
        DI: ssd1306::prelude::WriteOnlyDataCommand,
        SIZE: ssd1306::size::DisplaySize,
    {
        let mut divider;
        for i in 1..=4 {
            if i < 3 {
                divider = i;
            } else {
                divider = 8 - i + 1;
            }

            let _ = display.clear(BinaryColor::Off);
            self.eyes.blink(display, divider).await;
            display.flush().expect("Failed to flush display!");
        }
    }
}
