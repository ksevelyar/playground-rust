#![no_std]
#![no_main]

use esp_alloc as _;
use esp_hal::{clock::CpuClock, timer::timg::TimerGroup};
use music_box::{audio, user_input};
use panic_rtt_target as _;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    rtt_target::rtt_init_defmt!();
    defmt::info!("main: music-box starting");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 40960);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    let audio_out = audio::init(
        peripherals.I2S0,
        peripherals.DMA_CH0,
        peripherals.GPIO1,
        peripherals.GPIO2,
        peripherals.GPIO3,
        peripherals.GPIO4,
    );

    spawner.spawn(audio::play(audio_out).unwrap());
    spawner.spawn(user_input::lid_task(peripherals.GPIO0, peripherals.LPWR).unwrap());

    loop {
        Timer::after(Duration::from_secs(3600)).await;
    }
}
