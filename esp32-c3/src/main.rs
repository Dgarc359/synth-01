use embedded_midi::MidiIn;
use esp_idf_svc::hal;
use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::prelude::*;

use esp_idf_hal::delay::Ets;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();

    let uart_config = hal::uart::UartConfig {
        mode: hal::uart::config::Mode::UART,
        // TODO: get a better sense for what baudrate we want to use
        baudrate: hal::prelude::Hertz(31_250),
        // 9600 8 n 1 config
        data_bits: hal::uart::config::DataBits::DataBits8,
        // TODO: check what parity means again from wiki doc
        parity: hal::uart::config::Parity::ParityNone,
        stop_bits: hal::uart::config::StopBits::STOP1,
        flow_control: hal::uart::config::FlowControl::None,
        ..Default::default()
    };


    let rx: hal::gpio::Gpio1 = peripherals.pins.gpio1;
    let uart: hal::uart::UART0 = peripherals.uart0;

    let uart_driver: hal::uart::UartRxDriver = hal::uart::UartRxDriver::new(
        uart,
        rx,
        Option::<hal::gpio::Gpio1>::None,
        Option::<hal::gpio::Gpio1>::None,
        &uart_config
    ).expect("Unable to start uart driver!!!!!!!!");

    // read_midi(uart_driver);

    // we're outputting on the (physical) other side of the esp32-c3 board
    // just to make the wiring less cluttered
    let output_pin = hal::gpio::PinDriver::output(peripherals.pins.gpio2).expect("Failed to create output pin driver!!!");

    let middle_c_frequency = 261;
    let note_duration = 1000;

    output_frequency(middle_c_frequency, note_duration, output_pin);
}

// this will block the caller indefinitely.
fn read_midi(uart_driver: hal::uart::UartRxDriver<'_>) {
    let mut midi_in = MidiIn::new(uart_driver);

    loop {
        if let Ok(event) = midi_in.read() {
            println!("event: {:#?}", event)
        }
    }
}

fn output_frequency(frequency: u32, note_duration: u32, mut outpin: hal::gpio::PinDriver<'_, impl hal::gpio::OutputPin, hal::gpio::Output>) {
    // from arduino example, we subtract by 7 microseconds to make up
    // for digital write overhead. This might not be necessary
    // with esp
    let half_period = (500_000 / frequency) - 7;
    loop {
        outpin.set_high().expect("Couldn't set pin high");
        delay_microseconds(half_period);
        outpin.set_low();
        delay_microseconds(half_period);
    }
}


fn delay_microseconds(microseconds: u32) {
    Ets::delay_us(microseconds);
}
