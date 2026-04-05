use embedded_graphics::{
    mono_font::MonoTextStyle, pixelcolor::BinaryColor, prelude::*, text::Text,
};
use profont::PROFONT_24_POINT;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::units::FromValueType;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::sntp;
use esp_idf_svc::sys::EspError;
use esp_idf_svc::wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi};
use log::info;
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};
use std::time::{SystemTime, UNIX_EPOCH};

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASS");

fn main() -> Result<(), EspError> {
    let utc_offset: i32 = env!("UTC_OFFSET").parse().unwrap_or(180);

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
    let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().expect("OLED init failed");

    let text_style = MonoTextStyle::new(&PROFONT_24_POINT, BinaryColor::On);

    let mut last_second = 0u64;

    loop {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let second = now.as_secs();

        if second != last_second {
            let total_minutes = (second as i32 / 60) + utc_offset;
            let hours = ((total_minutes / 60) % 24) as u32;
            let minutes = (total_minutes % 60) as u32;

            let time_str = format!("{:02}:{:02}", hours, minutes);

            display.clear(BinaryColor::Off).unwrap();
            Text::new(&time_str, Point::new(24, 27), text_style)
                .draw(&mut display)
                .unwrap();
            display.flush().unwrap();

            last_second = second;
        }

        FreeRtos::delay_ms(100);
    }
}

fn wifi_create(
    ssid: &str,
    pass: &str,
    modem: esp_idf_svc::hal::modem::Modem<'static>,
    sysloop: EspSystemEventLoop,
) -> Result<EspWifi<'static>, EspError> {
    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;
    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop.clone())?;
    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.try_into().unwrap(),
        password: pass.try_into().unwrap(),
        ..Default::default()
    }))?;
    wifi.start()?;
    wifi.connect()?;
    wifi.wait_netif_up()?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    info!("Wifi DHCP info: {:?}", ip_info);
    Ok(esp_wifi)
}
