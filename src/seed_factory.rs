use std::time::{SystemTime, UNIX_EPOCH};

pub trait SeedFactory {
    fn create_seed(&self) -> u32;
}

pub fn default_factory() -> Box<dyn SeedFactory> {
    Box::new(OSSeedFactory {})
}

struct OSSeedFactory;

impl SeedFactory for OSSeedFactory {
    fn create_seed(&self) -> u32 {
        let now = SystemTime::now();
        now.duration_since(UNIX_EPOCH).expect("Error retrieving system time").as_micros() as u32
    }
}