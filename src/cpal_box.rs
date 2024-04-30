use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, Sample, SizedSample,
};

pub use crate::wav_decoder::decoder;

pub fn play_test(data: Vec<f32>) -> anyhow::Result<()> {
    let host = cpal::default_host();

    let device = host.default_output_device().unwrap();

    println!("Output device: {}", device.name()?);

    let config = device.default_output_config().unwrap();
    println!("Default output config: {:?}", config);

    match config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8>(&device, &config.into(), data),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), data),
        // cpal::SampleFormat::I24 => run::<I24>(&device, &config.into()),
        cpal::SampleFormat::I32 => run::<i32>(&device, &config.into(), data),
        // cpal::SampleFormat::I48 => run::<I48>(&device, &config.into()),
        cpal::SampleFormat::I64 => run::<i64>(&device, &config.into(), data),
        cpal::SampleFormat::U8 => run::<u8>(&device, &config.into(), data),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), data),
        // cpal::SampleFormat::U24 => run::<U24>(&device, &config.into()),
        cpal::SampleFormat::U32 => run::<u32>(&device, &config.into(), data),
        // cpal::SampleFormat::U48 => run::<U48>(&device, &config.into()),
        cpal::SampleFormat::U64 => run::<u64>(&device, &config.into(), data),
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), data),
        cpal::SampleFormat::F64 => run::<f64>(&device, &config.into(), data),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }
}

pub fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    data: Vec<f32>,
) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;

    let wav_data = decoder::get_mono();
    // println!("{:?}", wav);

    let next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        // (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
        // wav.get(sample_clock as usize).unwrap().to_owned()
    };

    let mut sample_index = 0;

    let next_value = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for sample in data.iter_mut() {
            let s = wav_data.get(sample_index).unwrap();
            *sample = *s;
            sample_index += 1;
        }
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(config, next_value, err_fn, None)?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(5000));

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
