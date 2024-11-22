struct Buffer {
    data: Vec<f32>,
}

impl Buffer {
    fn new(data: Vec<f32>) {
        Buffer { data }
    }

    fn get_cpal_sync(&self) {}
}
