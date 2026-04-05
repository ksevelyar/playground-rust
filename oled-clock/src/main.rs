use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::delay::{Ets, FreeRtos};
use esp_idf_svc::hal::gpio::{Level, Output, PinDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::nvs::*;
use esp_idf_svc::sntp;
use esp_idf_svc::sys::EspError;
use esp_idf_svc::wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi};
use log::info;
use std::time::{SystemTime, UNIX_EPOCH};

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASS");
const BLANK_DURATION: u32 = 500;
const DISPLAY_DURATION: u32 = 1500;

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

    let mut tube_value = [
        PinDriver::output(peripherals.pins.gpio0.degrade_output())?,
        PinDriver::output(peripherals.pins.gpio1.degrade_output())?,
        PinDriver::output(peripherals.pins.gpio2.degrade_output())?,
        PinDriver::output(peripherals.pins.gpio3.degrade_output())?,
    ];

    let mut tubes = [
        PinDriver::output(peripherals.pins.gpio7.degrade_output())?,
        PinDriver::output(peripherals.pins.gpio6.degrade_output())?,
        PinDriver::output(peripherals.pins.gpio5.degrade_output())?,
        PinDriver::output(peripherals.pins.gpio4.degrade_output())?,
    ];

    let mut digits = [0u8; 4];
    let mut last_update = 0u64;

    loop {
        maybe_update_state(utc_offset, &mut last_update, &mut digits);

        for (i, &digit) in digits.iter().enumerate() {
            set_all_tubes_low(&mut tubes)?;
            Ets::delay_us(BLANK_DURATION);

            set_tube_value(&mut tube_value, digit)?;
            select_tube(i, &mut tubes)?;
            Ets::delay_us(DISPLAY_DURATION);
        }
        FreeRtos::delay_ms(1);
    }
}

fn maybe_update_state(utc_offset: i32, last_update: &mut u64, digits: &mut [u8; 4]) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seconds = now.as_secs();
    if seconds == *last_update {
        return;
    }

    let total_minutes = seconds as i32 / 60 + utc_offset;
    let hours = ((total_minutes / 60) % 24) as u32;
    let minutes = (total_minutes % 60) as u32;
    *digits = [
        (hours / 10) as u8,
        (hours % 10) as u8,
        (minutes / 10) as u8,
        (minutes % 10) as u8,
    ];
    *last_update = seconds;
}

fn set_all_tubes_low(pins: &mut [PinDriver<'_, Output>]) -> Result<(), EspError> {
    for pin in pins.iter_mut() {
        pin.set_level(Level::Low)?;
    }
    Ok(())
}

fn select_tube(idx: usize, pins: &mut [PinDriver<'_, Output>]) -> Result<(), EspError> {
    for (i, pin) in pins.iter_mut().enumerate() {
        pin.set_level(if i == idx { Level::High } else { Level::Low })?;
    }
    Ok(())
}

fn set_tube_value(tube_value: &mut [PinDriver<'_, Output>], digit: u8) -> Result<(), EspError> {
    for (i, pin) in tube_value.iter_mut().enumerate() {
        let bit_set = (digit >> i) & 1 != 0;
        pin.set_level(if bit_set { Level::High } else { Level::Low })?;
    }
    Ok(())
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
