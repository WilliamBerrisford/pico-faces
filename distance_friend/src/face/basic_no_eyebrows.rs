use embassy_time::{Duration, Timer};
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};

use crate::face::eye::{BasicEye, Eye};

use super::Face;

pub struct BasicNoEyebrows {
    eyes: BasicEye,
    delay_secs: u64,
}

impl BasicNoEyebrows {
    pub fn basic_face() -> BasicNoEyebrows {
        let eye_base_x = 26;
        let eye_base_y = 40;
        let eye_height = 16;
        let eye_x_offset = 62;

        BasicNoEyebrows {
            eyes: BasicEye::new(eye_base_x, eye_base_y, eye_height, eye_x_offset),
            delay_secs: 6,
        }
    }
}

impl Face for BasicNoEyebrows {
    fn new() -> BasicNoEyebrows {
        let eye_base_x = 26;
        let eye_base_y = 40;
        let eye_height = 16;
        let eye_x_offset = 62;

        BasicNoEyebrows {
            eyes: BasicEye::new(eye_base_x, eye_base_y, eye_height, eye_x_offset),
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
