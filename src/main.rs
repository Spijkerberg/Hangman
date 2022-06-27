// Ideas
/*
- at first ask for difficulty ~ number of letters in word
&-
*/

use rand::Rng;
use std::{error::Error, fs, io};

fn main() {
    println!(
        "Let's play hangman!\nStart by selecting a difficulty:\n1) easy\n2) medium\n3) hard\n4) EXTREME"
    );

    // Get difficulty
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error: Failed to parse input.");

    let difficulty = match input.trim().parse::<u8>() {
        Ok(n) => n,
        Err(err) => panic!("Error ({err}). Failed to parse input ({input}). Try again"),
    };

    let word = match pick_word() {
        Ok(w) => w,
        Err(e) => panic!("Found error {e:?}"),
    };

    println!(
        "Selected difficulty is {difficulty}. The secret word is {}",
        &word
    );
}

fn pick_word() -> Result<String, Box<dyn Error>> {
    let whole_list = fs::read_to_string("data/words.txt")?;
    let word_vec = whole_list.split('\n').collect::<Vec<&str>>();
    let size = word_vec.len();
    let idx = rand::thread_rng().gen_range(0..=size);
    let word = match word_vec.get(idx) {
        Some(&word) => word,
        None => panic!(),
    };
    Ok(word.to_string())
}
