use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct FPSCounter {
    pub frame_times: VecDeque<Duration>,
    total_frame_time: Duration,
    last_frame_time: Instant,
}

impl FPSCounter {
    pub const MAX_FRAME_TIMES: usize = 100;

    pub fn new() -> Self {
        FPSCounter {
            frame_times: VecDeque::with_capacity(Self::MAX_FRAME_TIMES),
            total_frame_time: Default::default(),
            last_frame_time: Instant::now(),
        }
    }

    #[inline(always)]
    pub fn frame(&mut self) {
        let now = Instant::now();
        let frame_time = now - self.last_frame_time;

        if self.frame_times.len() >= Self::MAX_FRAME_TIMES {
            if let Some(oldest_frame_time) = self.frame_times.pop_front() {
                self.total_frame_time -= oldest_frame_time;
            }
        }

        self.frame_times.push_back(frame_time);
        self.total_frame_time += frame_time;
        self.last_frame_time = now;
    }

    #[inline(always)]
    pub fn fps(&self) -> u32 {
        let average_time = self.total_frame_time / self.frame_times.len() as u32;
        (1.0 / average_time.as_secs_f64()).round() as u32
    }
}
