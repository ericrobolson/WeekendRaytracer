use std::time::{Duration, Instant, SystemTime};

pub struct Clock {
    start: Instant,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        return Instant::now() - self.start;
    }

    pub fn stop_watch(&mut self) -> Duration {
        let elapsed = self.elapsed();
        self.start = Instant::now();

        elapsed
    }
}
