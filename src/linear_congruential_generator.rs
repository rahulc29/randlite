use crate::generator::Generator;
use crate::seed_factory::default_factory;
use std::num::Wrapping;

struct LinearCongruentialGenerator {
    a: Wrapping<u32>,
    c: Wrapping<u32>,
    modulo: Wrapping<u32>,
    current: Wrapping<u32>,
    max_value: Wrapping<u32>,
}

impl LinearCongruentialGenerator {
    fn next(&mut self) -> u32 {
        let mut next = (self.a * self.current + self.c) % self.modulo;
        next = next & self.max_value;
        self.current = next;
        next.0
    }
}

impl Generator for LinearCongruentialGenerator {
    fn gen_bool(&mut self) -> bool {
        return self.gen_uniform() > 0.5;
    }

    fn gen_u32(&mut self) -> u32 {
        return self.next();
    }

    fn gen_f32(&mut self) -> f32 {
        let next = self.next();
        unsafe {
            let ptr = &next as *const u32;
            let f32_ptr = ptr as *const f32;
            return *f32_ptr;
        };
    }

    fn gen_u64(&mut self) -> u64 {
        let a = self.gen_u32();
        let b = self.gen_u32() as u64;
        let a = a as u64;
        let a = a << 31;
        return a + b;
    }

    fn gen_f64(&mut self) -> f64 {
        let next = self.gen_u64();
        unsafe {
            let ptr = &next as *const u64;
            let f64_ptr = ptr as *const f64;
            return *f64_ptr;
        };
    }

    fn gen_uniform(&mut self) -> f32 {
        let mut the_int = self.gen_u32();
        if the_int == self.max_value.0 {
            the_int -= 1;
        }
        return (self.max_value.0 / the_int) as f32;
    }
}

pub fn bsd_generator() -> Box<dyn Generator> {
    Box::new(LinearCongruentialGenerator {
        a: Wrapping(1103515245),
        c: Wrapping(12345),
        modulo: Wrapping(1 << 31),
        current: Wrapping(default_factory().create_seed()),
        max_value: Wrapping(!1u32),
    })
}

pub fn microsoft_generator() -> Box<dyn Generator> {
    Box::new(LinearCongruentialGenerator {
        a: Wrapping(214013),
        c: Wrapping(2531011),
        modulo: Wrapping(1 << 31),
        current: Wrapping(default_factory().create_seed()),
        max_value: Wrapping(!1u32),
    })
}
