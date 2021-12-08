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
        let nanos = now
            .duration_since(UNIX_EPOCH)
            .expect("Error retrieving system time")
            .as_nanos();
        let to_return = (nanos & (!0u32 as u128)) as u32;
        return to_return;
    }
}