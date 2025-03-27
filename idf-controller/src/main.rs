#[allow(unused_imports)]
use esp_idf_svc::hal::delay::FreeRtos;
#[allow(unused_imports)]
use esp_idf_svc::hal::ledc::{
    config::TimerConfig, LedcDriver, LedcTimer, LedcTimerDriver, Resolution,
};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::prelude::*;

mod audio_out;
mod audio_in;
mod audio_waves;
mod note;
mod midi;
mod util;
mod envelope;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();

    let timer_driver = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &TimerConfig::default()
            .frequency(50.Hz())
            .resolution(Resolution::Bits14),
    )
    .unwrap();

    let (tx, rx) = channel::<crate::midi::SoundCommand>();

    let mut midi_in = MidiInput::new("midir reading input").expect("couldnt read midi inputs");
    midi_in.ignore(Ignore::None);

}