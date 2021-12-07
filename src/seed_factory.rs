use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub trait SeedFactory {
    fn create_seed(&self) -> u32;
}

impl dyn SeedFactory {
    // I'm not sure if `Box`ing this way is the idiomatic way
    // But for the moment : I'll just go along with it
    // Will refactor in the future
    pub fn default() -> Box<dyn SeedFactory> {
        Box::new(OSSeedFactory {})
    }
}

struct OSSeedFactory;

impl SeedFactory for OSSeedFactory {
    fn create_seed(&self) -> u32 {
        let now = SystemTime::now();
        now.duration_since(UNIX_EPOCH).expect("Error retrieving system time").as_micros() as u32
    }
}