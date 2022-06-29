// Ideas
/*
- at first ask for difficulty ~ number of letters in word

*/

use rand::seq::SliceRandom;
use std::{error::Error, fs, io};

fn main() {
    'main: loop {
        println!("Welcome to Hangman!\n");

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

        // loop until `death` or win
        'outer: loop {
            // check if player should be dead
            if lives <= 0 {
                println!("You lost the game.\nThe word was {word}");
                println!("\nPlay again? (Y/N)");
                'replay_loss: loop {
                    let guess: String = get_single_char(String::new().as_mut().to_string())
                        .expect("Something went wrong");
                    if guess.to_lowercase().contains('y') {
                        continue 'main;
                    } else if guess.to_lowercase().contains('n') {
                        break 'main;
                    } else {
                        println!("You what?");
                        continue 'replay_loss;
                    }
                }
            }
            // check if player won
            if !container.contains(&'.') {
                println!("You guessed it, you absolute beast!\nThe word was {word}");
                println!("\nPlay again? (Y/N)");
                'replay_win: loop {
                    let guess: String = get_single_char(String::new().as_mut().to_string())
                        .expect("Something went wrong");
                    if guess.to_lowercase().contains('y') {
                        continue 'main;
                    } else if guess.to_lowercase().contains('n') {
                        break 'main;
                    } else {
                        println!("You what?");
                        continue 'replay_win;
                    }
                }
            }

            // show current state of game
            show_state(&mut container, &mut guesses, &lives);

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
                        continue 'outer;
                    }
                }
                Err(_) => continue 'outer,
            };

            // update guesses
            if guesses.contains(&input) {
                'inner: loop {
                    println!("Are you absolutely sure that you want to waste a part of you on a character that you have already tried? (Y/N)");
                    let guess: String = get_single_char(String::new().as_mut().to_string())
                        .expect("Something went wrong");
                    if guess.to_lowercase().contains('y') {
                        println!("So be it, you shall lose a limb.");
                        lives -= 1;
                        continue 'outer;
                    } else if guess.to_lowercase().contains('n') {
                        println!("You shall be rewarded with mercy.");
                        continue 'outer;
                    } else {
                        println!("You what?");
                        continue 'inner;
                    }
                }
            } else {
                guesses.push(input);
            }
            // check if input is in the word
            if !word.contains(input) {
                println!("This character is not in the word. Guess again!");
                lives -= 1;
                continue 'outer;
            } else {
                // update container
                for (idx, val) in word.chars().enumerate() {
                    if input == val {
                        container.insert(idx, val);
                        container.remove(idx + 1);
                    }
                }
            }
        }
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

/// Given container of current state of the game and previously tried guesses
/// will print the state, guessed letters and number of lives
///
fn show_state(container: &mut [char], guesses: &mut [char], lives: &i8) {
    let mut result = String::new();
    for &el in container.iter() {
        result.push(el)
    }
    let mut letters = String::new();
    for &el in guesses.iter() {
        letters.push(el)
    }
    println!("You have {lives} lives!");
    println!("The word is: {}", result);
    if !letters.is_empty() {
        println!("Used letters: {}", letters)
    };
}

fn get_single_char(mut guess: String) -> Result<String, Box<dyn Error>> {
    match io::stdin().read_line(&mut guess) {
        Ok(n) => {
            if n != 3 {
                println!("Please enter a single char!")
            }
        }
        Err(e) => println!("Got error {e} while reading the line."),
    }
    Ok(guess)
}
