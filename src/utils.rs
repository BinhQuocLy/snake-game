use std::time::{Duration, SystemTime};

pub fn get_time_weight() -> f32 {
    let initial_time = SystemTime::UNIX_EPOCH;
    let duration: u64 = SystemTime::now()
        .duration_since(initial_time)
        .unwrap_or(Duration::new(0, 0))
        .as_secs();
    (duration % 100) as f32 / 100.0
}

pub fn is_new_second(time_weight: f32) -> bool {
    get_time_weight() != time_weight
}
