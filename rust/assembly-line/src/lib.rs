// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    // - `1` to `4`: 100% success rate.
    // - `5` to `8`: 90% success rate.
    // - `9` and `10`: 77% success rate.
    speed as f64
        * 221.0
        * match speed {
            0 => 0.0,
            1..=4 => 1.0,
            5..=8 => 0.9,
            9..=10 => 0.77,
            _ => 0.0,
        }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let hourly_rate = production_rate_per_hour(speed) as u32;
    hourly_rate / 60
}
