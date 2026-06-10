use defmt::info;
use embassy_time::Duration;
use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    i2s::master::{Channels, Config, DataFormat, I2s, I2sTx},
    time::Rate,
};

pub struct AudioClip {
    pub samples: &'static [u8],
    pub sample_rate_hz: u32,
}

static MOONLIGHT: AudioClip = AudioClip {
    samples: include_bytes!("../moonlight.pcm"),
    sample_rate_hz: 20000,
};

fn moonlight() -> &'static AudioClip {
    &MOONLIGHT
}

pub const CIRC_BUF_SIZE: usize = 32768;

pub struct AudioOut<'d> {
    pub tx: I2sTx<'d, esp_hal::Blocking>,
    pub sd: Output<'d>,
    pub buf: &'static mut [u8; CIRC_BUF_SIZE],
}

pub fn init<'a>(
    i2s0: esp_hal::peripherals::I2S0<'a>,
    dma_ch0: esp_hal::peripherals::DMA_CH0<'a>,
    gpio1: esp_hal::peripherals::GPIO1<'a>,
    gpio2: esp_hal::peripherals::GPIO2<'a>,
    gpio3: esp_hal::peripherals::GPIO3<'a>,
    gpio4: esp_hal::peripherals::GPIO4<'a>,
) -> AudioOut<'a> {
    let (_, _, tx_buffer, tx_descriptors) = esp_hal::dma_circular_buffers!(0, CIRC_BUF_SIZE);

    let i2s = I2s::new(
        i2s0,
        dma_ch0,
        Config::new_tdm_philips()
            .with_sample_rate(Rate::from_hz(20000))
            .with_data_format(DataFormat::Data16Channel16)
            .with_channels(Channels::MONO),
    )
    .expect("I2S init failed");

    let tx = i2s
        .i2s_tx
        .with_bclk(gpio2)
        .with_ws(gpio1)
        .with_dout(gpio3)
        .build(tx_descriptors);

    let sd = Output::new(gpio4, Level::Low, OutputConfig::default());

    AudioOut {
        tx,
        sd,
        buf: tx_buffer,
    }
}

#[embassy_executor::task]
pub async fn play(mut this: AudioOut<'static>) {
    info!("audio: starting playback");

    let clip = moonlight();
    let n = clip.samples.len().min(this.buf.len());
    this.buf[..n].copy_from_slice(&clip.samples[..n]);

    let mut transfer = this.tx.write_dma_circular(&*this.buf).unwrap();
    let mut cursor = n;
    this.sd.set_high();

    loop {
        if cursor >= clip.samples.len() {
            cursor = 0;
        }

        if let Ok(avail) = transfer.available()
            && avail > 0
        {
            let n = (clip.samples.len() - cursor).min(avail);
            transfer.push(&clip.samples[cursor..cursor + n]).unwrap();
            cursor += n;
        }

        embassy_time::Timer::after(Duration::from_micros(100)).await;
    }
}
