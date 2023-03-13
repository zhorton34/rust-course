use time::{Duration, PrimitiveDateTime};

pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    // Add one gigasecond (1,000,000,000 seconds) to the input date and time
    let one_gigasecond = Duration::seconds(1_000_000_000);
    
    start + one_gigasecond
}