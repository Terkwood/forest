use std::time::{Duration, SystemTime};

pub fn timed<T>(body: impl FnOnce() -> T) -> (T, Duration) {
    let start = SystemTime::now();
    let result = body();
    (result, start.elapsed().unwrap_or(Duration::new(0, 0)))
}
