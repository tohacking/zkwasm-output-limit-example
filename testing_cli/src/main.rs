use std::io::{ stdin, stdout, Write };
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use provable_game_logic::definition::SpinGameStates;
use provable_game_logic::spin::SpinGame;
use provable_game_logic::spin::SpinGameTrait;

fn print_game_state(stdout: &mut std::io::Stdout) {
    let final_game_state: SpinGameStates = SpinGame::get_game_state();
    writeln!(stdout, "{}\r", final_game_state).unwrap();
}

const GAME_INSTRUCTIONS: &str =
    "Use <- (left arrow) and -> (right arrow) to control the game, q to quit.\r";

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    const previousState_score: u64 = 0;
    const previousState_total_number_seen: u64 = 0;
    const previousState_total_capture_tries: u64 = 0;
    const previousState_current_number: u64 = 0;
    const previousState_random_seed: u64 = 0;

    SpinGame::initialize_game(
        SpinGameStates::new(
            previousState_score,
            previousState_total_number_seen,
            previousState_total_capture_tries,
            previousState_current_number,
            previousState_random_seed
        )
    );

    writeln!(stdout, "{}", GAME_INSTRUCTIONS).unwrap();
    print_game_state(&mut stdout);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char(' ') => SpinGame::step(0),
            Key::Char('q') => {
                break;
            } // Quit the game
            _ => SpinGame::step(1),
        }
        print_game_state(&mut stdout);
    }
}
