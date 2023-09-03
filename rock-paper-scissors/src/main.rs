use std::fs;
use rock_paper_scissors_lib::game_options::GameOptions;

fn main() {
    // read the inputs from the file
    let game_contents = fs::read_to_string("rock-paper-scissors/rock-paper-scissors.txt").expect("Failed to read the file");

    let mut total_score = 0;
    // each line represents a round, first char is their move second is mine.
    game_contents.split("\n").for_each(|row|{
        // final row will be nothing
        if row != "" {
            let row_no_spaces = row.replace(" ", ""); 
            let mut chars = row_no_spaces.chars();
            let elf = chars.next().unwrap();
            let myself = chars.next().unwrap();
            println!("row: {}, elf: {}, myself {}", row, elf, myself);
            let elf = convert_char_to_type(elf);
            let myself = convert_char_to_type(myself);
            total_score += determine_winner_and_score(elf, myself);
        }
    });
    println!("{}", total_score);
}

pub fn convert_char_to_type(char: char) -> GameOptions {
    if char == 'A' || char == 'X' {
        GameOptions::Rock
    } else if char == 'B' || char == 'Y' {
        GameOptions::Paper
    } else if char == 'C' || char == 'Z' {
        GameOptions::Scissors
    } else {
        panic!("Invalid char entered")
    }
}

pub fn determine_winner_and_score(elf: GameOptions, myself: GameOptions) -> u16 {
    match elf {
        GameOptions::Rock => {
            match myself {
                GameOptions::Rock => return 4,
                GameOptions::Paper => return 8,
                GameOptions::Scissors => return 3,
            }
        },
        GameOptions::Scissors => {
            match myself {
                GameOptions::Rock => return 7,
                GameOptions::Paper => return 2,
                GameOptions::Scissors => return 6,
            }
        },
        GameOptions::Paper => {
            match myself {
                GameOptions::Rock => return 1,
                GameOptions::Paper => return 5,
                GameOptions::Scissors => return 9,
            }
        }
    }
}
