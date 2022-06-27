// Ideas
/*
- at first ask for difficulty ~ number of letters in word
&-
*/

use rand::Rng;
use std::{error::Error, fs, io};

fn main() {
    // println!(
    //     "Let's play hangman!\nStart by selecting a difficulty:\n1) easy\n2) medium\n3) hard\n4) EXTREME"
    // );

    // // Get difficulty
    // let mut input = String::new();
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Error: Failed to parse input.");

    // let difficulty = match input.trim().parse::<u8>() {
    //     Ok(n) => n,
    //     Err(err) => panic!("Error ({err}). Failed to parse input ({input}). Try again"),
    // };

    // Get word
    let word = match pick_word() {
        Ok(w) => w,
        Err(e) => panic!("Found error {e:?}"),
    };

    // Create container for the individual letters
    let mut container: Vec<char> = vec!['.'; word.len() - 1];

    // Debug
    println!("The word is {word}");

    // loop until `death` or win
    loop {
        // Get input string from user
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(n) => {
                if n != 3 {
                    println!("Please enter a single char!")
                }
            }
            Err(e) => println!("Got error {e} while reading the line."),
        }

        // Convert string input to char
        let input = match guess.trim().parse::<char>() {
            Ok(input) => {
                if input.is_alphabetic() {
                    input
                } else {
                    println!("Please enter an alphabetical char!");
                    continue;
                }
            }
            Err(_) => continue,
        };

        // Check if input is in the word
        if !word.contains(input) {
            println!("This character is not in the word. Guess again!");
            // print current word and guesses
            show_state(&mut container);
            continue;
        } else {
            println!("This character is in the word!");
            // update container
            for (idx, val) in word.chars().enumerate() {
                if input == val {
                    container.insert(idx, val);
                    container.remove(idx + 1);
                }
            }
            // print updated word
            show_state(&mut container);
        }
        if !container.contains(&'.') {
            break;
        }
    }
}

/// Given list of words pick a word.
///
/// TODO:
///
/// ~ Use difficulty measure to provide different types of words (length, number of different characters).
///
/// ~ Use Vec.choose instead of get.
///
/// ~ Clean up this mess.
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

/// Given container of current state of the game print the word and guesses
///
fn show_state(container: &mut Vec<char>) {
    //}, guesses: &Vec<char>) {
    let mut result = String::new();
    for &el in container.clone().iter() {
        result.push(el)
    }
    println!("{}", result)
}
