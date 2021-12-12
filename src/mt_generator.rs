use crate::generator::Generator;
use std::num::Wrapping;
use crate::seed_factory::default_factory;

pub struct MTGenerator {
    matrix_a: usize,
    n: usize,
    m: usize,
    upper_mask: usize,
    lower_mask: usize,
    mask_b: usize,
    mask_c: usize,
    state: Vec<u64>,
}

pub fn mt_generator() -> Box<dyn Generator> {
    Box::new(MTGenerator::new(default_factory().create_seed()))
}

impl MTGenerator {
    pub fn new(seed: u32) -> Self {
        let n = 624usize;
        let m = 397usize;
        let matrix_a = 0x9908b0dfusize;
        let upper_mask = 0x80000000usize;
        let lower_mask = 0x7fffffffusize;
        let mask_b = 0x9d2c5680usize;
        let mask_c = 0xefc60000usize;
        let mut state = vec![0u64; n as usize];
        state[0] = seed as u64;
        Self {
            n,
            m,
            matrix_a,
            upper_mask,
            lower_mask,
            mask_b,
            mask_c,
            state,
        }
    }
    fn init_state(&self, seed: u64) -> Vec<u64> {
        let mut to_return = vec![0u64; self.n];
        to_return[0] = seed;
        for i in 1..self.n {
            to_return[i as usize] = (Wrapping(to_return[(i - 1) as usize]) * Wrapping(69069)).0;
        }
        return to_return;
    }
    fn next(&mut self) -> u64 {
        let mut state = self.init_state(self.state[0]);
        let mut to_return = 0u64;
        let to_xor = |y: u64| {
            if y % 2 == 1 {
                self.matrix_a as u64
            } else {
                0u64
            }
        };
        let next_answer = |x: u64, y: u64| {
            (x & self.upper_mask as u64) | (y & self.lower_mask as u64)
        };
        let diff = self.n - self.m;
        for i in 0..diff {
            // update the result
            to_return = next_answer(state[i], state[i + 1]);
            // update the state
            state[i] = state[i + self.m] ^ (to_return >> 1);
            state[i] ^= to_xor(to_return);
        }
        for i in diff..(self.n - 1) {
            to_return = next_answer(state[i], state[i + 1]);
            state[i] = state[i - diff] ^ (to_return >> 1);
            state[i] ^= to_xor(to_return);
        }
        to_return = (state[self.n - 1] & self.upper_mask as u64) | (state[0] & self.lower_mask as u64);
        state[self.n - 1] = state[self.m - 1] ^ (to_return >> 1);
        state[self.n - 1] ^= to_xor(to_return);
        to_return = state[0];
        // temper u
        to_return ^= to_return >> 11;
        // temper s
        to_return ^= (to_return << 7) & self.mask_b as u64;
        // temper t
        to_return ^= (to_return << 15) & self.mask_c as u64;
        // temper l
        to_return ^= to_return >> 18;
        self.state = state;
        return to_return;
    }
    pub fn state(&self) -> &Vec<u64> {
        &self.state
    }
}

impl Generator for MTGenerator {
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