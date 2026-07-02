use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;

fn main() {
    // Required patch links for ESP-IDF
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP-IDF logging facility
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("=================================");
    log::info!(" EvolGenesis Firmware v0.1.0");
    log::info!(" ESP32 Hardware Bring-up");
    log::info!("=================================");

    // Take ownership of the peripherals (safe singleton)
    let peripherals = Peripherals::take().unwrap();

    // GPIO2 = onboard LED on most ESP32 DevKit boards
    // If your LED doesn't blink, try GPIO 4 or check your board's pinout
    let mut led = PinDriver::output(peripherals.pins.gpio2).unwrap();

    log::info!("LED on GPIO2 — starting blink loop...");

    let mut count = 0u32;

    loop {
        count += 1;

        led.set_high().unwrap();
        log::info!("Blink #{} — LED ON", count);
        FreeRtos::delay_ms(500);

        led.set_low().unwrap();
        log::info!("Blink #{} — LED OFF", count);
        FreeRtos::delay_ms(500);
    }
}
