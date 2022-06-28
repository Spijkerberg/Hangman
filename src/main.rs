// Ideas
/*
- at first ask for difficulty ~ number of letters in word
&-
*/

use rand::seq::SliceRandom;
use std::{error::Error, fs, io};

fn main() {
    println!("Welcome to Hangman!\nBegin by entering a character:");

    // start live with 6 lives
    let mut lives: i8 = 6;

    // // get difficulty
    // let mut input = String::new();
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Error: Failed to parse input.");

    // let difficulty = match input.trim().parse::<u8>() {
    //     Ok(n) => n,
    //     Err(err) => panic!("Error ({err}). Failed to parse input ({input}). Try again"),
    // };

    // get word
    let word = match pick_word() {
        Ok(w) => w,
        Err(e) => panic!("Found error {e:?}"),
    };

    // create container for the individual letters and a container for the guesses
    let mut container: Vec<char> = vec!['.'; word.len() - 1];
    let mut guesses: Vec<char> = vec![];

    // debug
    println!("The word is {word}");

    // loop until `death` or win
    loop {
        // get input string from user
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(n) => {
                if n != 3 {
                    println!("Please enter a single char!")
                }
            }
            Err(e) => println!("Got error {e} while reading the line."),
        }

        // convert string input to char
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

        // update guesses
        if guesses.contains(&input) {
            println!("You have already tried this character.");
            continue;
        } else {
            guesses.push(input);
        }
        // check if input is in the word
        if !word.contains(input) {
            println!("This character is not in the word. Guess again!");
            lives -= 1;
            // print current word and guesses
            show_state(&mut container, &mut guesses, &lives);
            continue;
        } else {
            // update container
            for (idx, val) in word.chars().enumerate() {
                if input == val {
                    container.insert(idx, val);
                    container.remove(idx + 1);
                }
            }

            // print updated word
            show_state(&mut container, &mut guesses, &lives);
        }

        // check if player should be dead
        if lives < 0 {
            println!("You have lost the game lmao.");
            break;
        }

        if !container.contains(&'.') {
            println!("You guessed the word, you absolute beast!");
            break;
        }
    }
}

fn _update_guesses(guesses: &mut Vec<char>, input: char, lives: &mut i8) {
    // update guesses
    if guesses.contains(&input) {
        loop {
            println!("Are you absolutely sure that you want to waste a part of you on a character that you have already tried? (Y/N)");
            let mut guess = String::new();
            match io::stdin().read_line(&mut guess) {
                Ok(n) => {
                    if n != 3 {
                        println!("Please enter a single char!")
                    }
                }
                Err(e) => println!("Got error {e} while reading the line."),
            }
            if guess.to_lowercase().contains('y') {
                println!("So be it, you shall lose a limb.");
                *lives -= 1;
                break;
            } else if guess.to_lowercase().contains('n') {
                println!("You shall be rewarded with mercy.");
                break;
            } else {
                println!("You what?");
                continue;
            }
        }
    } else {
        guesses.push(input);
    }
}

/// Given list of words pick a word.
///
/// TODO:
///
/// ~ Use difficulty measure to provide different types of words (length, number of different characters).
///
fn pick_word() -> Result<String, Box<dyn Error>> {
    // import word list
    let word_list = fs::read_to_string(r"data\words.txt")?;
    let words = word_list.split('\n').collect::<Vec<&str>>();

    // choose word from list
    let mut rng = rand::thread_rng();
    let word = match words.choose(&mut rng) {
        Some(&chosen_word) => chosen_word,
        _ => panic!("Something went wrong!"),
    };
    Ok(word.to_string())
}

/// Given container of current state of the game print the word and guesses
///
fn show_state(container: &mut Vec<char>, guesses: &mut Vec<char>, lives: &i8) {
    let mut result = String::new();
    for &el in container.clone().iter() {
        result.push(el)
    }
    let mut letters = String::new();
    for &el in guesses.clone().iter() {
        letters.push(el)
    }
    print!("{}", 27 as char);
    println!("You have {lives} lives!");
    println!("The word is: {}", result);
    println!("Used letters: {}", letters);
}
