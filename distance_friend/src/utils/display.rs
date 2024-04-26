use defmt::info;
use ssd1306::{
    mode::{BufferedGraphicsMode, DisplayConfig},
    prelude::WriteOnlyDataCommand,
    size::DisplaySize,
    Ssd1306,
};

pub async fn init_display<DI, SIZE>(display: &mut Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>)
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    info!("Attempting to initialise display.");

    display.init().expect("Failed to init display");
    display.flush().expect("Failed to flush display");

    info!("Display initialised.");
}
