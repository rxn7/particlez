pub struct FpsCounter {
    last_frame_time: std::time::Instant,
    frame_count: u32,
    fps: u32
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            last_frame_time: std::time::Instant::now(),
            frame_count: 0,
            fps: 0,
        }
    }

    pub fn fps(&self) -> u32 {
        self.fps
    }

    pub fn tick(&mut self) {
        self.frame_count += 1;
        if self.last_frame_time.elapsed() > std::time::Duration::from_secs(1) {
            self.fps = self.frame_count;
            self.frame_count = 0;
            self.last_frame_time = std::time::Instant::now();
        }
    }
}
