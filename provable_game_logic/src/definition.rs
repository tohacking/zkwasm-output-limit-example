use crate::rng::LCGRandGen;
use serde::{ Deserialize, Serialize };
use std::fmt;
use wasm_bindgen::prelude::*;

#[derive(Clone, Serialize)]
#[wasm_bindgen]
pub struct SpinGameStates {
    // define your game state here
    pub score: u64,
    pub total_number_seen: u64,
    pub total_capture_tries: u64,
    pub current_number: u64,
    pub random_seed: LCGRandGen,
}

#[wasm_bindgen]
impl SpinGameStates {
    #[wasm_bindgen(constructor)]
    pub fn new(
        score: u64,
        total_number_seen: u64,
        total_capture_tries: u64,
        current_number: u64,
        random_seed: u64
    ) -> SpinGameStates {
        SpinGameStates {
            score: score,
            total_number_seen: total_number_seen,
            total_capture_tries: total_capture_tries,
            current_number: current_number,
            random_seed: LCGRandGen::new(random_seed),
        }
    }
}

impl fmt::Display for SpinGameStates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GameState {{ score: {}, count: {}, tries: {}, current_number: {}, seed: {} }}",
            self.score,
            self.total_number_seen,
            self.total_capture_tries,
            self.current_number,
            self.random_seed.seed
        )
    }
}
