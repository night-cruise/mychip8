use crate::chip8::{Duration, Instant};

/// the clock of chip-8
pub struct Clock {
    period: Duration, // nanoseconds of a clock period
    offset: Instant,  // elapsed clock period
}

impl Clock {
    /// create a clock instance
    pub fn new(freq: u16) -> Clock {
        Clock {
            period: Duration::from_nanos(1000000000 / freq as u64),
            offset: Instant::now(),
        }
    }

    /// clock tick
    /// return true if a clock period has elapsed. otherwise, return false
    pub fn tick(&mut self) -> bool {
        if self.offset.elapsed() >= self.period {
            self.offset += self.period;
            true
        } else {
            false
        }
    }
}
