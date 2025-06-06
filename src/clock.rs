use std::time::{Duration, Instant};

/// Simple clock to track elapsed time.
#[derive(Default)]
pub struct Clock {
    start: Option<Instant>,
    active: bool,
}

impl Clock {
    pub fn new() -> Self {
        Clock::default()
    }

    /// Starts the clock if not already running.
    pub fn start(&mut self) {
        if self.active {
            return;
        }

        self.start = Some(Instant::now());
        self.active = true;
    }

    /// Stops the clock (if running) and records the elapsed time.
    pub fn stop(&mut self) -> Duration {
        let elapsed = self.elapsed();

        self.active = false;
        self.start = None;

        elapsed
    }

    /// Returns the clock's elapsed time.
    pub fn elapsed(&self) -> Duration {
        if let Some(start) = self.start {
            return start.elapsed();
        }

        Duration::default()
    }

    pub fn is_running(&self) -> bool {
        self.active
    }
}
