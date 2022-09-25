use chrono::{DateTime, Duration, TimeZone, Utc};

struct RateLimiter {
    period_limit: i32,
    period_in_seconds: i32,
    previous_requests: Vec<DateTime<Utc>>,
}

impl RateLimiter {
    fn new(n: i32, t: i32) -> Self {
        RateLimiter {
            period_limit: n,
            period_in_seconds: t,
            previous_requests: Vec::new(),
        }
    }

    fn should_allow(&mut self, timestamp: i32) -> bool {
        // let now = Instant::);

        let now = Utc.timestamp(timestamp as i64, 0);
        let mut previous_requests = self.previous_requests.clone();
        let mut updated_previous_requests = Vec::new();
        let threshold_time = now
            .checked_sub_signed(Duration::seconds(self.period_in_seconds as i64))
            .unwrap();

        // Get list of requests within time period
        for (index, request) in previous_requests.clone().iter().enumerate() {
            // Probably can improve that ^
            if request > &threshold_time && previous_requests.len() > 0 {
                updated_previous_requests.append(&mut previous_requests.split_off(index));
            }
        }
        if updated_previous_requests.len() < self.period_limit as usize {
            updated_previous_requests.push(now);
            self.previous_requests = updated_previous_requests;
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod rate_limiter_tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut rl = RateLimiter::new(3, 5);
        let r1 = rl.should_allow(1);
        let r2 = rl.should_allow(1);
        let r3 = rl.should_allow(2);
        let r4 = rl.should_allow(3);
        let r5 = rl.should_allow(8);

        assert!(r1);
        assert!(r2);
        assert!(r3);
        assert!(!r4);
        assert!(r5);
    }
}
