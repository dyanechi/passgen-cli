use rand::{rngs::ThreadRng, Rng};

pub fn rand_number(rng: &mut ThreadRng, min: usize, max: usize) -> usize {
    rng.gen_range(min..max)
}