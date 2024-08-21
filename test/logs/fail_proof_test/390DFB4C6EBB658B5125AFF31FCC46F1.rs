use provable_game_logic::definition::SpinGameStates;
use provable_game_logic::spin::SpinGame;
use provable_game_logic::spin::SpinGameTrait;
use provable_game_logic::rng::LCGRandGen;
use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::wasm_input;
use zkwasm_rust_sdk::wasm_output;

/*
    PUBLIC INPUTS marked by `wasm_input`: e.g wasm_input(1) specifies a public input of type u64
    PRIVATE INPUTS marked by `wasm_input`: e.g wasm_input(0) specifies a priva  te input of type u64
    PUBLIC OUTPUTS marked by `wasm_output`: e.g wasm_output(var) specifies an output `var` of type u64
*/
#[wasm_bindgen]
pub fn zkmain() -> i64 {
    // --- META transaction data ---
    let transactionNonce: u64 = unsafe { wasm_input(1) };
    let gameId: u64 = unsafe { wasm_input(1) };
    let inputSigner0: u64 = unsafe { wasm_input(1) };
    let inputSigner1: u64 = unsafe { wasm_input(1) };
    let inputSigner2: u64 = unsafe { wasm_input(1) };

    // --- GAME transaction data ---
    let previousState_score: u64 = unsafe { wasm_input(1) };
    let previousState_total_number_seen: u64 = unsafe { wasm_input(1) };
    let previousState_total_capture_tries: u64 = unsafe { wasm_input(1) };
    let previousState_current_number: u64 = unsafe { wasm_input(1) };
    let previousState_random_seed: u64 = unsafe { wasm_input(1) };

    SpinGame::initialize_game(SpinGameStates {
        score: previousState_score,
        total_number_seen: previousState_total_number_seen,
        total_capture_tries: previousState_total_capture_tries,
        current_number: previousState_current_number,
        random_seed: LCGRandGen::new(previousState_random_seed),
    });

    // specify the private inputs
    let private_inputs_length = unsafe { wasm_input(0) };

    for _i in 0..private_inputs_length {
        let _private_input = unsafe { wasm_input(0) };
        SpinGame::step(_private_input);
    }

    unsafe {
        let final_game_state: SpinGameStates = SpinGame::get_game_state();
        zkwasm_rust_sdk::dbg!("final_game_state: {}\n", final_game_state);

        // --- GAME transaction data output ---
        wasm_output(final_game_state.score as u64);
        // wasm_output(final_game_state.total_number_seen as u64);
        // wasm_output(final_game_state.total_capture_tries as u64);
        // wasm_output(final_game_state.current_number as u64);
        // wasm_output(final_game_state.random_seed.seed as u64);
    }

    0
}
// RESULT: success
