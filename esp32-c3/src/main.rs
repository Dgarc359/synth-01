use embedded_midi::MidiIn;
use esp_idf_svc::hal;
use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::prelude::*;

use log::info;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");

    info!("creating peripherals");
    let peripherals = Peripherals::take().unwrap();

    info!("creating uart config");
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


    info!("creating rx gpio and uart");
    let rx: hal::gpio::Gpio1 = peripherals.pins.gpio1;
    let uart: hal::uart::UART0 = peripherals.uart0;

    info!("creating uart driver");
    let uart_driver: hal::uart::UartRxDriver = hal::uart::UartRxDriver::new(
        uart,
        rx,
        Option::<hal::gpio::Gpio1>::None,
        Option::<hal::gpio::Gpio1>::None,
        &uart_config
    ).expect("Unable to start uart driver!!!!!!!!");

    info!("getting outpin");
    // read_midi(uart_driver);

    // we're outputting on the (physical) other side of the esp32-c3 board
    // just to make the wiring less cluttered
    let output_pin = hal::gpio::PinDriver::output(peripherals.pins.gpio2).expect("Failed to create output pin driver!!!");

    info!("setting frequency");
    let middle_c_frequency = 261;
    let note_duration = 1000;


    info!("Attempting to read from uart driver");

    // loop {
    //     info!("busy boxing :)");
    // }
    // loop {
    //     let buf: &mut [u8] = &mut vec![];
    //     let t = uart_driver.read(buf, 0).unwrap();
    //     log::info!("{:#?}", t);
    // }

    output_frequency(middle_c_frequency, note_duration, output_pin);
    // read_midi(uart_driver);
}

// this will block the caller indefinitely.
fn read_midi(uart_driver: hal::uart::UartRxDriver<'_>) {
    let mut midi_in = MidiIn::new(uart_driver);

    loop {
        // info!("checking for event");
        if let Ok(event) = midi_in.read() {
            // println!("event: {:#?}", event)
            info!("got event!");
        }
    }
}

fn output_frequency(frequency: u32, note_duration: u32, mut outpin: hal::gpio::PinDriver<'_, impl hal::gpio::OutputPin, hal::gpio::Output>) {
    // from arduino example, we subtract by 7 microseconds to make up
    // for digital write overhead. This might not be necessary
    // with esp
    let half_period = (500_000 / frequency) - 7;
    let cycles = (( frequency as u64 * note_duration as u64) / 1000) as u64;
    loop {
        for i in 0..cycles {
            outpin.set_high().expect("Couldn't set pin high");
            delay_microseconds(half_period);
            outpin.set_low().expect("Couldn't set pin to LOW");
            delay_microseconds(half_period);
        }
    }
}


fn delay_microseconds(microseconds: u32) {
    Ets::delay_us(microseconds);
}
