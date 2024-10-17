use std::time::{Duration, Instant};

#[derive(Default)]
pub struct Clock {
    start: Option<Instant>,
    elapsed: Duration,
    active: bool,
}

impl Clock {
    pub fn new() -> Self {
        Clock::default()
    }

    pub fn start(&mut self) {
        if self.active {
            return;
        }

        self.start = Some(Instant::now());
        self.active = true;
    }

    pub fn stop(&mut self) {
        if !self.active {
            return;
        }

        if let Some(start) = self.start {
            self.elapsed = start.elapsed();
            self.start = None;
        }

        self.active = false;
        self.start = None;
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }
}
