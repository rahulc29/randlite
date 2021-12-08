mod generator;
mod linear_congruential_generator;
mod seed_factory;

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use crate::generator::Generator;
    use crate::linear_congruential_generator::{bsd_generator, microsoft_generator};
    use crate::seed_factory;

    #[test]
    fn generate_seed() {
        let factory = seed_factory::default_factory();
        let seed_a = factory.create_seed();
        // we need just a few microseconds of delay
        // to generate separate seeds
        thread::sleep(Duration::from_millis(1));
        let seed_b = factory.create_seed();
        assert_ne!(seed_a, seed_b);
    }

    #[test]
    fn generate_seeds_instantly() {
        let factory = seed_factory::default_factory();
        // with nanoseconds the time difference b/w the two invocations is captured
        // thereby yielding different seeds for every invocation
        let seed_a = factory.create_seed();
        let seed_b = factory.create_seed();
        assert_ne!(seed_a, seed_b);
    }

    impl dyn Generator {
        fn test_gen_uniform(&mut self) {
            let uniform = self.gen_uniform();
            assert!(uniform < 1f32);
            assert!(uniform >= 0f32);
        }
        // this test has a failure probability of 2 ** (-32)
        fn test_gen_u32(&mut self) {
            let a = self.gen_u32();
            let b = self.gen_u32();
            assert_ne!(a, b);
        }
        // this test has a failure probability of 2 ** (-32)
        fn test_gen_f32(&mut self) {
            let a = self.gen_f32();
            let b = self.gen_f32();
            assert_ne!(a, b);
        }
        // this test has a failure probability of 2 ** (-64)
        fn test_gen_u64(&mut self) {
            let a = self.gen_u64();
            let b = self.gen_u64();
            assert_ne!(a, b);
        }
        // this test has a failure probability of 2 ** (-64)
        fn test_gen_f64(&mut self) {
            let a = self.gen_f64();
            let b = self.gen_f64();
            assert_ne!(a, b);
        }
    }

    #[test]
    fn lcg_gen_uniform() {
        let mut microsoft = microsoft_generator();
        microsoft.test_gen_uniform();
        let mut bsd = bsd_generator();
        bsd.test_gen_uniform();
    }

    #[test]
    fn lcg_gen_u32() {
        let mut microsoft = microsoft_generator();
        let mut bsd = bsd_generator();
        microsoft.test_gen_u32();
        bsd.test_gen_u32();
    }

    #[test]
    fn lcg_gen_u64() {
        let mut microsoft = microsoft_generator();
        let mut bsd = bsd_generator();
        microsoft.test_gen_u64();
        bsd.test_gen_u64();
    }

    #[test]
    fn lcg_gen_f32() {
        let mut microsoft = microsoft_generator();
        let mut bsd = bsd_generator();
        microsoft.test_gen_f32();
        bsd.test_gen_f32();
    }

    #[test]
    fn lcg_gen_f64() {
        let mut microsoft = microsoft_generator();
        let mut bsd = bsd_generator();
        microsoft.test_gen_f64();
        bsd.test_gen_f64();
    }
}
