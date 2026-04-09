use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::sys::esp_wifi_set_max_tx_power;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::sntp;
use esp_idf_svc::wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi};

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};

use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::{PinDriver, Pull};
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::units::FromValueType;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::sys::EspError;
use log::info;
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};
use std::time::SystemTime;

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASS");

fn main() -> Result<(), EspError> {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

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
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().expect("OLED init failed");

    let reed = PinDriver::input(peripherals.pins.gpio2, Pull::Up)?;

    let mut steps: u32 = 0;
    let mut last_trigger_ms: u64 = 0;
    let mut last_reed_high: bool = true;

    draw_steps(&mut display, steps).ok();
    display.flush().ok();

    loop {
        let now_ms = get_now_ms();
        let current_low = reed.is_low();

        if current_low && last_reed_high {
            if now_ms.saturating_sub(last_trigger_ms) > 50 {
                last_trigger_ms = now_ms;
                steps = steps.wrapping_add(1);
                draw_steps(&mut display, steps).ok();
                display.flush().ok();
            }
        }

        last_reed_high = !current_low;
        FreeRtos::delay_ms(50);
    }
}

fn get_now_ms() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn draw_steps<D>(display: &mut D, steps: u32) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let val = steps % 10000;
    let d1 = ((val / 1000) % 10) as u8;
    let d2 = ((val / 100) % 10) as u8;
    let d3 = ((val / 10) % 10) as u8;
    let d4 = (val % 10) as u8;

    display.clear(BinaryColor::Off)?;
    draw_digit(display, d1, 0)?;
    draw_digit(display, d2, 31)?;
    draw_digit(display, d3, 62)?;
    draw_digit(display, d4, 93)?;
    Ok(())
}

fn draw_digit<D>(display: &mut D, digit: u8, x_offset: i32) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let segments = match digit {
        0 => 63,
        1 => 6,
        2 => 91,
        3 => 79,
        4 => 102,
        5 => 109,
        6 => 125,
        7 => 7,
        8 => 127,
        9 => 111,
        _ => 0,
    };
    let style = PrimitiveStyle::with_fill(BinaryColor::On);

    if segments & 1 != 0 {
        Rectangle::new(Point::new(x_offset + 12, 0), Size::new(12, 4))
            .into_styled(style)
            .draw(display)?;
    }
    if segments & 2 != 0 {
        Rectangle::new(Point::new(x_offset + 24, 4), Size::new(4, 28))
            .into_styled(style)
            .draw(display)?;
    }
    if segments & 4 != 0 {
        Rectangle::new(Point::new(x_offset + 24, 36), Size::new(4, 28))
            .into_styled(style)
            .draw(display)?;
    }
    if segments & 8 != 0 {
        Rectangle::new(Point::new(x_offset + 12, 60), Size::new(12, 4))
            .into_styled(style)
            .draw(display)?;
    }
    if segments & 16 != 0 {
        Rectangle::new(Point::new(x_offset + 8, 36), Size::new(4, 28))
            .into_styled(style)
            .draw(display)?;
    }
    if segments & 32 != 0 {
        Rectangle::new(Point::new(x_offset + 8, 4), Size::new(4, 28))
            .into_styled(style)
            .draw(display)?;
    }
    if segments & 64 != 0 {
        Rectangle::new(Point::new(x_offset + 12, 30), Size::new(12, 4))
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
