// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate = if speed <= 4 {
        1.0
    } else if speed <= 8 {
        0.9
    } else {
        0.77
    };
    let production_rate = speed as f64 * 221.0 * success_rate;
    
    production_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let production_rate = production_rate_per_hour(speed) / 60.0;
    let working_items = production_rate.floor() as u32;

    working_items
}
