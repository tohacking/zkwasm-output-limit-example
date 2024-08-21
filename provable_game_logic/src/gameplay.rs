use crate::definition::SpinGameStates;
use crate::rng::LCGRandGen;
use crate::spin::SpinGame;
use crate::spin::SpinGameTrait;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

pub static NUMBER_THRESHOLD: u64 = 100;
pub static MAX_STEP_COUNT: u64 = 2000;
pub static MAX_SCORE: u64 = 5;

pub static GAME_STATE: Lazy<Mutex<SpinGameStates>> = Lazy::new(||
    Mutex::new(SpinGameStates::new(0, 0, 0, 0, 0))
);

impl SpinGameTrait for SpinGame {
    /* STATEFUL FUNCTIONS This defines the initialization of the game*/
    fn initialize_game(args: SpinGameStates) {
        let mut game_state = GAME_STATE.lock().unwrap();

        game_state.score = args.score;
        game_state.total_number_seen = args.total_number_seen;
        game_state.total_capture_tries = args.total_capture_tries;

        game_state.current_number = args.current_number;
        game_state.random_seed = args.random_seed;
    }

    /* STATEFUL FUNCTIONS This is defines the logic when player moves one step/entering one command*/

    fn step(input: u64) {
        let mut game_state = GAME_STATE.lock().unwrap();

        match input {
            // 0 means a capture command, the only command that can be executed in the game
            0 => {
                if game_state.current_number < NUMBER_THRESHOLD {
                    game_state.score += 1;
                }

                game_state.current_number = game_state.random_seed.randint(NUMBER_THRESHOLD * 2);
                // game_state.current_number = 0;

                game_state.total_capture_tries += 1;
            }
            // skips the current number
            1 => {
                game_state.current_number = game_state.random_seed.randint(NUMBER_THRESHOLD * 2);
                // game_state.current_number = 0;
            }

            _ => {
                panic!("Invalid command");
            }
        }

        game_state.total_number_seen += 1;
    }

    /* PURE FUNCTION This function returns the game state, to be used in Rust and Zkmain */
    fn get_game_state() -> SpinGameStates {
        let game = GAME_STATE.lock().unwrap().clone();
        return game;
    }
}
