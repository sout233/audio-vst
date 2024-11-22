pub mod decoder {
    use hound;

    pub fn get_stereo() -> (Vec<f32>, Vec<f32>) {
        let mut reader = hound::WavReader::open("test48k_32.wav").unwrap();

        let mut left_channel_data = Vec::new();
        let mut right_channel_data = Vec::new();

        let mut is_left = true;

        for (index, sample) in reader.samples::<i32>().enumerate() {
            let sample = sample.unwrap();
            if is_left {
                let left_sample = sample as f32 / i32::MAX as f32;
                left_channel_data.push(left_sample);
            } else {
                let right_sample = sample as f32 / i32::MAX as f32;
                right_channel_data.push(right_sample);
            }
            is_left = !is_left;
            // let right_sample = (sample >> 16) as f32 / i32::MAX as f32;
        }

        let spec = &reader.spec();
        println!("{:?}", spec);

        (left_channel_data.to_owned(), right_channel_data.to_owned())
    }

    pub fn get_mono() -> Vec<f32> {
        let mut reader = hound::WavReader::open("test48k_32.wav").unwrap();

        let mut mono_channel_data = Vec::new();

        for sample in reader.samples::<i32>() {
            let sample = sample.unwrap();
            let mono_sample = sample as f32 / i32::MAX as f32;
            mono_channel_data.push(mono_sample);
        }

        let spec = &reader.spec();
        println!("{:?}", spec);

        mono_channel_data
    }

    pub fn to_mono(stereo_wav: (Vec<f32>, Vec<f32>)) -> Vec<f32> {
        let vec1 = stereo_wav.0;
        let vec2 = stereo_wav.1;
    
        let mut result = Vec::with_capacity(vec1.len() + vec2.len());
    
        for (a, b) in vec1.iter().zip(vec2.iter()) {
            result.push(*a);
            result.push(*b);
        }
    
        // 如果vec1或vec2较长，将剩余的元素添加到结果中
        result.extend(vec1.iter().skip(vec2.len()));
        result.extend(vec2.iter().skip(vec1.len()));

        result
    }
}
