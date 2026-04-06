use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::sntp;
use esp_idf_svc::sys::EspError;
use esp_idf_svc::wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi};
use log::info;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::SmartLedsWrite;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASS");

const NUM_LEDS: usize = 24;
const BRIGHTNESS: u8 = 8;
const SATURATION: u8 = 200;

const PALETTES: &[(u8, u8)] = &[
    (160, 220), // cyan → blue
    (220, 255), // magenta → cyan
    (0,   40),  // red → orange
    (40,  80),  // orange → yellow
    (80, 160),  // green → cyan
];

const TRANSITION_DURATION_MS: u32 = 5000;
const FRAME_MS: u32 = 50;

fn lerp_hue(a: u8, b: u8, t: f32) -> u8 {
    let diff = ((b as i16 - a as i16 + 256) % 256) as f32;
    let hue = (a as f32 + diff * t) % 256.0;
    hue as u8
}

fn palette_hue(palette: &(u8, u8), position: f32) -> u8 {
    let (start, end) = *palette;
    lerp_hue(start, end, position)
}

#[allow(deprecated)]
fn main() -> Result<(), EspError> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    let rmt_channel = peripherals.rmt.channel0;
    let led_pin = peripherals.pins.gpio5;
    let mut leds = Ws2812Esp32Rmt::new(rmt_channel, led_pin)
        .expect("Failed to initialize WS2812B LEDs");

    let sysloop = EspSystemEventLoop::take()?;
    let _nvs = EspDefaultNvsPartition::take()?;
    let _wifi = wifi_create(SSID, PASSWORD, peripherals.modem, sysloop)?;
    let _sntp = sntp::EspSntp::new_default()?;
    info!("SNTP initialized");

    let mut palette_idx = 0usize;
    let mut transition = 0.0f32;

    loop {
        let current = &PALETTES[palette_idx];
        let next = &PALETTES[(palette_idx + 1) % PALETTES.len()];

        let pixels = (0..NUM_LEDS).map(|i| {
            let pos = i as f32 / NUM_LEDS as f32;

            let hue_now = palette_hue(current, pos);
            let hue_tgt = palette_hue(next, pos);

            let hue = lerp_hue(hue_now, hue_tgt, transition);

            hsv2rgb(Hsv {
                hue,
                sat: SATURATION,
                val: BRIGHTNESS,
            })
        });

        leds.write(pixels).unwrap();

        transition += FRAME_MS as f32 / TRANSITION_DURATION_MS as f32;

        if transition >= 1.0 {
            transition = 0.0;
            palette_idx = (palette_idx + 1) % PALETTES.len();
        }

        FreeRtos::delay_ms(FRAME_MS);
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
