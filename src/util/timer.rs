use std::time::{SystemTime, UNIX_EPOCH};

pub struct Timer {
    last_time: u64,
    frame_leap: u64,
    delta: u64,
    current_frames: u64,
    pub fps: u64
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            last_time: get_elapsed_time(),
            frame_leap: get_elapsed_time(),
            delta: get_elapsed_time(),
            current_frames: 0,
            fps: 0,
        }
    }

    pub fn frame_leap(&self) -> u32 {
        self.frame_leap as u32
    }

    pub fn tick(&mut self) {
        let last_frame = self.delta;
        self.delta = get_elapsed_time();
        self.frame_leap = self.delta - last_frame;

        if self.delta - self.last_time > 1000 {
            self.fps = self.current_frames;
            self.current_frames = 0;
            self.last_time = self.delta;
        }

        self.current_frames += 1;
    }
}

fn get_elapsed_time() -> u64 {
    let start = SystemTime::now();

    let since_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_epoch.as_secs() * 1000 + since_epoch.subsec_nanos() as u64 / 1_000_000
}