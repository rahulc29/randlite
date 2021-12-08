use crate::generator::Generator;

struct LinearCongruentialGenerator {
    a: u32,
    c: u32,
    modulo: u32,
    current: u32,
    max_value: u32,
}

impl LinearCongruentialGenerator {
    fn next(&mut self) -> u32 {
        let mut next = (self.a * self.current + self.c) % self.modulo;
        next = next & self.max_value;
        self.current = next;
        next
    }
}

impl Generator for LinearCongruentialGenerator {
    fn gen_bool(&mut self) -> bool {
        return self.gen_f32() > 0.5;
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
}

// pub static BSD_GENERATOR: LinearCongruentialGenerator = LinearCongruentialGenerator {
//     a: 1103515245,
//     c: 12345,
//     modulo: (1 << 31),
//     current: default_factory().create_seed(),
//     max_value: !1u32,
// };
//
// pub static MICROSOFT_GENERATOR: LinearCongruentialGenerator = LinearCongruentialGenerator {
//     a: 214013,
//     c: 2531011,
//     modulo: (1 << 31),
//     current: default_factory().create_seed(),
//     max_value: !1u32,
// };
