use std::thread;

mod cpal_box;
mod rodio_box;
mod vst_box;
mod wav_decoder;

fn main() {
    thread::spawn(move || {
        let data = wav_decoder::decoder::get_stereo();
        let sample = cpal_box::play_test(wav_decoder::decoder::to_mono(data));
    });


    let mut plugin = vst_box::Box::new();
    plugin.init(48000.0, 48000 / 100);
    let _ = plugin.show_editor();

    loop {}
}

#[allow(dead_code)]
fn archive_main() {
    // let sample = cpal_box::play_test();
    // rodio_box::play();
    let data = wav_decoder::decoder::get_stereo();
    let mut plugin = vst_box::Box::new();
    plugin.init(48000.0, 48000 / 100);
    let _ = plugin.show_editor();
    println!("{:?}", data);
}
