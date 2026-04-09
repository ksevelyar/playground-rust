use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::sys::esp_wifi_set_max_tx_power;
use esp_idf_svc::hal::units::FromValueType;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::sntp;
use esp_idf_svc::sys::EspError;
use esp_idf_svc::wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi};

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};
use log::info;
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};
use std::time::{SystemTime, UNIX_EPOCH};

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASS");

fn main() -> Result<(), EspError> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;
    let _nvs = EspDefaultNvsPartition::take()?;

    let _wifi = wifi_create(SSID, PASSWORD, peripherals.modem, sysloop)?;

    let _sntp = sntp::EspSntp::new_default()?;
    info!("SNTP initialized");

    let sda = peripherals.pins.gpio3;
    let scl = peripherals.pins.gpio4;
    let i2c = I2cDriver::new(
        peripherals.i2c0,
        sda,
        scl,
        &I2cConfig::new().baudrate(400u32.kHz().into()),
    )?;

    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate90)
        .into_buffered_graphics_mode();

    display.init().expect("OLED init failed");

    let mut last_second = 0u64;

    loop {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let second = now.as_secs();

        if second != last_second {
            let last_digit = (second % 10) as u8;

            display.clear(BinaryColor::Off).unwrap();

            draw_fullscreen_digit(&mut display, last_digit).unwrap();

            display.flush().unwrap();
            last_second = second;
        }

        FreeRtos::delay_ms(1000);
    }
}

fn draw_fullscreen_digit<D>(display: &mut D, digit: u8) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    // Bitmask for 7-segment layout: A=1, B=2, C=4, D=8, E=16, F=32, G=64
    let segments = match digit {
        0 => 63,  // A, B, C, D, E, F
        1 => 6,   // B, C
        2 => 91,  // A, B, D, E, G
        3 => 79,  // A, B, C, D, G
        4 => 102, // B, C, F, G
        5 => 109, // A, C, D, F, G
        6 => 125, // A, C, D, E, F, G
        7 => 7,   // A, B, C
        8 => 127, // All segments
        9 => 111, // A, B, C, D, F, G
        _ => 0,
    };

    let style = PrimitiveStyle::with_fill(BinaryColor::On);

    // Coordinate mapping designed perfectly for Width: 32, Height: 128
    // A (Top)
    if segments & 1 != 0 {
        Rectangle::new(Point::new(8, 0), Size::new(16, 8))
            .into_styled(style)
            .draw(display)?;
    }
    // B (Top Right)
    if segments & 2 != 0 {
        Rectangle::new(Point::new(24, 8), Size::new(8, 52))
            .into_styled(style)
            .draw(display)?;
    }
    // C (Bottom Right)
    if segments & 4 != 0 {
        Rectangle::new(Point::new(24, 68), Size::new(8, 52))
            .into_styled(style)
            .draw(display)?;
    }
    // D (Bottom)
    if segments & 8 != 0 {
        Rectangle::new(Point::new(8, 120), Size::new(16, 8))
            .into_styled(style)
            .draw(display)?;
    }
    // E (Bottom Left)
    if segments & 16 != 0 {
        Rectangle::new(Point::new(0, 68), Size::new(8, 52))
            .into_styled(style)
            .draw(display)?;
    }
    // F (Top Left)
    if segments & 32 != 0 {
        Rectangle::new(Point::new(0, 8), Size::new(8, 52))
            .into_styled(style)
            .draw(display)?;
    }
    // G (Middle)
    if segments & 64 != 0 {
        Rectangle::new(Point::new(8, 60), Size::new(16, 8))
            .into_styled(style)
            .draw(display)?;
    }

    Ok(())
}

// NOTE: reddit.com/r/arduino/comments/1dl6atc/esp32c3_boards_cant_connect_to_wifi_when_plugged
fn fix_breadboard_wifi() {
    unsafe {
        esp_wifi_set_max_tx_power(34);
        esp_idf_svc::hal::sys::esp_wifi_set_ps(esp_idf_svc::hal::sys::wifi_ps_type_t_WIFI_PS_NONE);
    }
}

fn wifi_create(
    ssid: &str,
    pass: &str,
    modem: esp_idf_svc::hal::modem::Modem<'static>,
    sysloop: EspSystemEventLoop,
) -> Result<EspWifi<'static>, EspError> {
    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;
    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop)?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.try_into().unwrap(),
        password: pass.try_into().unwrap(),
        ..Default::default()
    }))?;

    wifi.start()?;
    fix_breadboard_wifi();

    wifi.connect()?;
    wifi.wait_netif_up()?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    info!("Wifi DHCP info: {:?}", ip_info);

    Ok(esp_wifi)
}
