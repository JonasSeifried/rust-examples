use std::io;
use unicode_segmentation::UnicodeSegmentation;

use crate::game_data::{GameData, GameState};
pub mod game_data;

fn main() {
    let mut game_data = GameData::init();
    game_data.set_random_word();
    println!("Welcome to Hangman");

    /*
    println!("Please enter the word that will be guessed");
    let mut scaned = false;
    while !scaned {
        scaned = game_data.scan_word();
    }
    */
    println!(
        "Lets begin!!\nThe word has {} letters",
        game_data.get_word_len()
    );
    println!("{}", game_data.word_to_string());
    game_loop(&mut game_data);
}

fn game_loop(game_data: &mut GameData) {
    let guess = get_guess();
    if guess.eq("") {
        return game_loop(game_data);
    }
    //execute!(stdout(), Clear(ClearType::All)).expect("Failed to Clear Terminal");
    match game_data.handle_guess(&guess) {
        -1 => println!(
            "You already tried {} and it was not part of the word",
            guess
        ),
        0 => println!("Great {} was part of the word", guess),
        1 => println!("You already tried {} and it was part of the word", guess),
        2 => println!("{} was not part of the word", guess),
        _ => (),
    }
    println!("{}\n", game_data.word_to_string());
    if game_data.get_wrongly_guessed_count() > 0 {
        println!("{}", game_data.hangman_to_string());
    }

    match game_data.get_state() {
        GameState::InGame => game_loop(game_data),
        GameState::GameWon => {
            println!("You Won");
            println!("The word was: {}", game_data.get_word())
        }
        GameState::GameOver => {
            println!("Gameover, you lost");
            println!("The word was: {}", game_data.get_word())
        }
    }
}

fn get_guess() -> String {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading line");
    return guess.graphemes(true).next().unwrap_or("").to_string();
}
