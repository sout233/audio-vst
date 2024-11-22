use std::{
    thread,
    time::{Duration, Instant},
};

use winit::event_loop::{EventLoop, EventLoopProxy};

mod cpal_box;
mod rodio_box;
mod vst_box;
mod wav_decoder;

const SAMPLE_RATE: usize = 48000;
const BLOCK_SIZE: usize = SAMPLE_RATE / 100;

fn main() {
    let (left_wav_data, right_wav_data) = wav_decoder::decoder::get_stereo();
    let mut block_pos = 0;
    let mut plugin = vst_box::Box::new();

    plugin.init(48000.0, 48000 / 100);

    let event_loop = EventLoop::<(Vec<f32>, Vec<f32>)>::with_user_event().unwrap();

    let event_loop_proxy = event_loop.create_proxy();

    let get_samples = move || {
        
    };


    thread::spawn(move || {
        let data = wav_decoder::decoder::get_stereo();
        let _ = cpal_box::play_test(wav_decoder::decoder::to_mono(data));
    });

    thread::spawn(move || {
        let start = Instant::now();
        let mut t = 0;

        loop {
            let mut left_samples: Vec<f32> = vec![0f32; BLOCK_SIZE];
            let mut right_samples: Vec<f32> = vec![0f32; BLOCK_SIZE];

            for i in 0..BLOCK_SIZE {
                let t_sec = (t + i) as f64 / SAMPLE_RATE as f64;
                // samples[i] = f64::sin(t_sec * 2.0 * std::f64::consts::PI * 440.0) as f32 / 10.0;
                left_samples[i] = *left_wav_data
                    .get(i + block_pos * BLOCK_SIZE)
                    .unwrap_or(&0.0);
            }

            for i in 0..BLOCK_SIZE {
                right_samples[i] = *right_wav_data
                    .get(i + block_pos * BLOCK_SIZE)
                    .unwrap_or(&0.0);
            }

            block_pos += 1;

            let _ = event_loop_proxy.send_event((left_samples, right_samples));

            t += BLOCK_SIZE;

            let wait_until = start + Duration::from_millis(t as u64 * 1000 / SAMPLE_RATE as u64);
            let now = Instant::now();
            if wait_until > now {
                thread::sleep(wait_until - now);
            }
        }
    });

    plugin.show_editor(event_loop);

    loop {}
}

#[allow(dead_code)]
fn archive_main() {
    // let sample = cpal_box::play_test();
    // rodio_box::play();
    let data = wav_decoder::decoder::get_stereo();
    let mut plugin = vst_box::Box::new();
    plugin.init(48000.0, 48000 / 100);
    // let _ = plugin.show_editor();
    println!("{:?}", data);
}
