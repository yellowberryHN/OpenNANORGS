use std::fmt::Debug;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_chacha::rand_core::RngCore;


pub trait RNGSystem: Debug {
    fn rand(&mut self, max: Option<u32>) -> u32;

    fn get_seed(&self) -> u32;
}

#[derive(Debug)]
pub struct LegacyRNG {
    state: u32,
    initial_seed: u32
}

impl LegacyRNG {
    pub fn new(seed: u32) -> LegacyRNG {
        LegacyRNG { state: seed, initial_seed: seed }
    }
}

impl RNGSystem for LegacyRNG {
    fn rand(&mut self, max: Option<u32>) -> u32 {
        self.state = self.state.wrapping_mul(16807) % 0x7fffffff;
        match max {
            None => {self.state}
            Some(_) => {self.state % (max.unwrap()+1)}
        }
    }

    fn get_seed(&self) -> u32 {
        self.initial_seed
    }
}

#[derive(Debug)]
pub struct ModernRNG {
    rng: ChaCha20Rng,
    initial_seed: u32
}

impl ModernRNG {
    pub fn new(seed: u32) -> ModernRNG {
        ModernRNG { rng: ChaCha20Rng::seed_from_u64(seed as u64), initial_seed: seed }
    }
}

impl RNGSystem for ModernRNG {
    fn rand(&mut self, max: Option<u32>) -> u32 {
        match max {
            None => {self.rng.next_u32()}
            Some(max) => {self.rng.gen_range(0..=max)}
        }
    }

    fn get_seed(&self) -> u32 {
        self.initial_seed
    }
}