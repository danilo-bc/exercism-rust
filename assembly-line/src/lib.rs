pub fn production_rate_per_hour(speed: u8) -> f64 {
    let succ_rate;
    match speed{
        1..=4 => succ_rate = 1.0,
        5..=8 => succ_rate = 0.9,
        9..=10 => succ_rate = 0.77,
        _ => succ_rate = 0.0,
    }
    (speed as f64) * 221.0 * succ_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
