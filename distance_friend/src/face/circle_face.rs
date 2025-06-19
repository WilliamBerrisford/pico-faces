use embassy_time::{Duration, Timer};
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};

use crate::face::{
    eye::{CircleEye, Eye},
    eyebrow::{BasicEyebrow, EyeBrow},
};

use super::{
    Face,
    mouth::{Mouth, Smile},
};

const EYE_BASE_X: i32 = 26;
const EYE_BASE_Y: i32 = 40;
const EYE_HEIGHT: u32 = 8;
const EYE_X_OFFSET: i32 = 62;

const EYEBROW_BASE_X: i32 = 18;
const EYEBROW_BASE_Y: i32 = 5;
const EYEBROW_X_OFFSET: i32 = 62;

const MOUTH_X: i32 = 42;
const MOUTH_Y: i32 = 24;

pub struct CircleFace {
    eyes: CircleEye,
    eyebrows: BasicEyebrow,
    mouth: Smile,
    delay_secs: u64,
}

impl Face for CircleFace {
    fn new() -> CircleFace {
        CircleFace {
            eyes: CircleEye::new(EYE_BASE_X, EYE_BASE_Y, EYE_HEIGHT, EYE_X_OFFSET),
            eyebrows: BasicEyebrow::new(EYEBROW_BASE_X, EYEBROW_BASE_Y, EYEBROW_X_OFFSET),
            mouth: Smile::new(MOUTH_X, MOUTH_Y),
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
            display.clear(BinaryColor::Off);
            self.eyes.normal(display).await;
            self.eyebrows.normal(display).await;
            self.mouth.normal(display).await;
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

            display.clear(BinaryColor::Off);
            self.eyebrows.normal(display).await;
            self.eyes.blink(display, divider).await;
            self.mouth.normal(display).await;
            display.flush().expect("Failed to flush display!");
        }
    }
}
