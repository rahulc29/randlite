use std::num::Wrapping;
use crate::generator::Generator;
use crate::seed_factory::{default_factory, SeedFactory};

pub struct AcornGenerator {
    k: usize,
    modulo: u64,
    y: Vec<Vec<u64>>,
    n: usize,
}

pub fn acorn_generator() -> Box<dyn Generator> {
    Box::new(AcornGenerator::new(20, (1 << 60), default_factory()))
}

impl AcornGenerator {
    pub fn new(k: usize, modulo: u64, factory: Box<dyn SeedFactory>) -> Self {
        let mut seeds = Vec::with_capacity(k + 1);
        for _i in 0..=k {
            let seed_a = factory.create_seed();
            let seed_b = factory.create_seed();
            let seed = ((seed_a as u64) << 32u64) | seed_b as u64;
            seeds.push((Wrapping(seed) * Wrapping(37u64)).0 % modulo);
        }
        let mut y = Vec::new();
        y.push(seeds);
        y.push(vec![0u64; k + 1]);
        Self {
            k,
            modulo,
            y,
            n: 1,
        }
    }
    fn next(&mut self) -> u64 {
        self.y[self.n][0] = self.y[self.n - 1][0];
        for i in 1..=self.k {
            self.y[self.n][i] = Wrapping(self.y[self.n][i - 1] + self.y[self.n - 1][i]).0 as u64 % self.modulo;
        }
        self.n += 1;
        return self.y[self.n - 1][self.k];
    }
}

impl Generator for AcornGenerator {
    fn gen_bool(&mut self) -> bool {
        self.gen_uniform() > 0.5f32
    }

    fn gen_u32(&mut self) -> u32 {
        (self.next() & u32::MAX as u64) as u32
    }

    fn gen_f32(&mut self) -> f32 {
        let next = self.next();
        unsafe {
            let ptr = (&next as *const u64) as *const f32;
            return *ptr;
        }
    }

    fn gen_u64(&mut self) -> u64 {
        self.next()
    }

    fn gen_f64(&mut self) -> f64 {
        let next = self.next();
        unsafe {
            let ptr = (&next as *const u64) as *const f64;
            return *ptr;
        }
    }

    fn gen_uniform(&mut self) -> f32 {
        let int = self.next();
        let max = u64::MAX;
        return int as f32 / max as f32;
    }
}