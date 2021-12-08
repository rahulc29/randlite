pub trait Generator {
    fn gen_bool(&mut self) -> bool;
    fn gen_u32(&mut self) -> u32;
    fn gen_f32(&mut self) -> f32;
    fn gen_u64(&mut self) -> u64;
    fn gen_f64(&mut self) -> f64;
    fn gen_uniform(&mut self) -> f32;
}