use std::time::Instant;

pub struct Limiter {
    rate_limit: u32,
    per_seconds: u64,
    tokens: u32,
    last_refresh_time: Instant,
}

impl Limiter {
    pub fn new(rate_limit: u32, per_seconds: u64) -> Self {
        Limiter {
            rate_limit,
            per_seconds,
            tokens: rate_limit,
            last_refresh_time: Instant::now(),
        }
    }

    fn refresh_token(&mut self) {
        let current_time = Instant::now();
        let time_passed = current_time.duration_since(self.last_refresh_time);
        let time_passed_secs =
            time_passed.as_secs() as f64 + f64::from(time_passed.subsec_nanos()) / 1_000_000_000.0;
        let new_tokens = time_passed_secs * (self.rate_limit as f64 / self.per_seconds as f64);
        self.tokens = self.tokens.min(self.rate_limit) + new_tokens as u32;
        self.last_refresh_time = current_time;
    }

    pub fn acquire(&mut self, token: u32) -> bool {
        self.refresh_token();
        if self.tokens >= token {
            self.tokens -= token;
            true
        } else {
            false
        }
    }
}
