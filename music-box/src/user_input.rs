use defmt::info;
use embassy_time::{Duration, Timer};
use esp_hal::{
    gpio::{Input, InputConfig, Pull, RtcPin, RtcPinWithResistors},
    peripherals::{GPIO0, LPWR},
    rtc_cntl::Rtc,
    rtc_cntl::sleep::{RtcSleepConfig, WakeSource, WakeTriggers},
};

#[embassy_executor::task]
pub async fn lid_task(mut gpio0: GPIO0<'static>, lpwr: LPWR<'static>) {
    let mut reed = Input::new(gpio0.reborrow(), InputConfig::default().with_pull(Pull::Up));
    Timer::after(Duration::from_millis(50)).await;

    if reed.is_low() {
        info!("input: lid closed at boot, entering deep sleep");
        enter_deep_sleep(gpio0, lpwr);
    }

    info!("input: lid open, waiting for lid close");
    reed.wait_for_low().await;
    info!("input: lid closed, entering deep sleep");
    enter_deep_sleep(gpio0, lpwr);
}

fn enter_deep_sleep(gpio0: GPIO0<'static>, lpwr: LPWR<'static>) -> ! {
    info!("input: entering deep sleep, wake on GPIO0 high level");

    let mut rtc = Rtc::new(lpwr);
    let wake = GpioWakeSource { pin: gpio0 };
    rtc.sleep_deep(&[&wake]);
}

struct GpioWakeSource {
    pin: GPIO0<'static>,
}

// FIXME: internal pull-up is 45kOhm, draws 73mA during sleep; replace with 1Mohm external pull-up to drop current to 3.3mA
impl WakeSource for GpioWakeSource {
    fn apply(
        &self,
        _rtc: &Rtc<'_>,
        triggers: &mut WakeTriggers,
        _sleep_config: &mut RtcSleepConfig,
    ) {
        triggers.set_gpio(true);

        self.pin.rtcio_pullup(true);
        self.pin.rtcio_pulldown(false);
        self.pin.rtcio_pad_hold(true);

        unsafe {
            self.pin.apply_wakeup(true, 5);
        }
    }
}
