use rock_paper_scissors_lib::{game_options::GameOptions, game_state::GameState};
use std::fs;

fn main() {
    // read the inputs from the file
    let game_contents = fs::read_to_string("rock-paper-scissors/rock-paper-scissors.txt")
        .expect("Failed to read the file");

    let mut total_score = 0;
    // each line represents a round, first char is their move second is mine.
    game_contents.split("\n").for_each(|row| {
        // final row will be nothing
        if row != "" {
            let row_no_spaces = row.replace(" ", "");
            let mut chars = row_no_spaces.chars();
            let elf = chars.next().unwrap();
            let myself = chars.next().unwrap();
            let elf = convert_elf_char(elf);
            total_score += determine_my_score(elf, myself);
        }
    });
    println!("{}", total_score);
}

pub fn convert_elf_char(char: char) -> GameOptions {
    if char == 'A' {
        GameOptions::Rock
    } else if char == 'B' {
        GameOptions::Paper
    } else if char == 'C' {
        GameOptions::Scissors
    } else {
        panic!("Invalid char entered")
    }
}

pub fn convert_my_char(char: char) -> GameState {
    if char == 'X' {
        GameState::Lose
    } else if char == 'Y' {
        GameState::Tie
    } else if char == 'Z' {
        GameState::Win
    } else {
        panic!("Invalid char entered")
    }
}

pub fn determine_my_score(elf_choice: GameOptions, my_char: char) -> u16 {
    let state = convert_my_char(my_char);
    match elf_choice {
        GameOptions::Rock => {
            match state {
                GameState::Win => 8,
                GameState::Tie => 4,
                GameState::Lose => 3,
            }
        },
        GameOptions::Paper => {
            match state {
                GameState::Win => 9,
                GameState::Tie => 5,
                GameState::Lose => 1,
            }
        },
        GameOptions::Scissors => {
            match state {
                GameState::Win => 7,
                GameState::Tie => 6,
                GameState::Lose => 2,
            }
        }
    }
}
