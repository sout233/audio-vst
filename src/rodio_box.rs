extern crate rodio;

use std::{fs::File, io::BufReader, thread, time::Duration};

use rodio::{source::SineWave, Decoder, OutputStream, Sink, Source};

pub fn play() {
    thread::spawn(|| {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        // Get a output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open("test.wav").unwrap());
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();
        sink.append(source);

        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        sink.sleep_until_end();
    });
}
