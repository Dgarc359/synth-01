use embedded_midi::MidiIn;
use esp_idf_svc::hal;
use esp_idf_svc::hal::prelude::*;

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

    let mut midi_in = MidiIn::new(uart_driver);

    loop {
        if let Ok(event) = midi_in.read() {
            println!("event: {:#?}", event)
        }
    }
}
