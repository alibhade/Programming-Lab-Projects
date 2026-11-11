use rand::Rng; //importing random number generator
use std::io::{self, Write}; //importing i/o utilities for input/output streams and prompt flushing

// defining constants (M = sequence length, N = max digit)
const M: usize = 4;
const N: u8 = 6;

// giving Score some abilities which are that it can be printed for debuggin and that two Scores can be compared using ==
#[derive(Debug, PartialEq, Eq)]
pub struct Score {
    pub bulls: usize,
    pub cows: usize,
}

// evaluation of the users guess against the target sequence and returns the count of bulls and cows
pub fn evaluate_guess(target: &[u8], guess: &[u8]) -> Score {
    // checking that the guess is of the correct length
    assert_eq!(
        guess.len(),
        M,
        "Guess length must match game configuration M"
    );

    let mut bulls = 0;
    // sequence to hold unmatched elements (needed for calculating cows)
    let mut target_unmatched = Vec::new();
    let mut guess_unmatched = Vec::new();

    // counting bulls (exact matches in position and value)
    for i in 0..M {
        if guess[i] == target[i] {
            bulls += 1;
        } else {
            //this will keep track of the remaining non-matching pairs
            target_unmatched.push(target[i]);
            guess_unmatched.push(guess[i]);
        }
    }

    // counting cows (correct value, wrong position) among remaining unmatched numbers
    let mut cows = 0;
    for g_num in guess_unmatched {
        if let Some(pos) = target_unmatched.iter().position(|&t_num| t_num == g_num) {
            cows += 1;
            // remove matched element from target slice so that theres no double counting repeated numbers
            target_unmatched.remove(pos);
        }
    }

    Score { bulls, cows }
}

// generating a random target sequence of length M with numbers ranging from 1 to N
fn generate_target() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..M).map(|_| rng.gen_range(1..=N)).collect()
}

// parsing and validating input text from the user
fn parse_input(input: &str) -> Result<Option<Vec<u8>>, String> {
    let trimmed = input.trim();

    //checking for exit signal
    if trimmed.eq_ignore_ascii_case("exit") {
        return Ok(None);
    }

    //splitting input line by whitespace into tokenized strings
    let parts: Vec<&str> = trimmed.split_whitespace().collect();

    //checking length
    if parts.len() != M {
        return Err(format!(
            "Invalid length: Expected exactly {} numbers separated by whitespace, found {}.",
            M,
            parts.len()
        ));
    }

    //validate and convert each token into 8-bit int
    let mut guess = Vec::with_capacity(M);
    for part in parts {
        match part.parse::<u8>() {
            //parsed correctly and valid
            Ok(num) if num >= 1 && num <= N => guess.push(num),
            //parsed correctlly but outside allowed bounds
            Ok(num) => {
                return Err(format!(
                    "Number out of bounds: '{}' is not between 1 and {}.",
                    num, N
                ))
            }
            //failed string to int conversion
            Err(_) => {
                return Err(format!(
                    "Invalid input: '{}' is not a valid integer.",
                    part
                ))
            }
        }
    }

    Ok(Some(guess))
}

// running game loop session
fn play_game() -> bool {
    //return true if player won by finding target seq or false if user aborts
    let target = generate_target();
    println!("\n--- New Game Started! ---");
    println!(
        "Guess the secret sequence of {} numbers (each between 1 and {}).",
        M, N
    );
    println!("Type 'exit' anytime to leave current game.\n");

    let mut turn = 1;
    loop {
        print!("Turn {} > Enter guess: ", turn);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input. Please try again.");
            continue;
        }

        //process user input
        match parse_input(&input) {
            Ok(Some(guess)) => {
                let score = evaluate_guess(&target, &guess);
                println!(
                    "-> Result: {} Bull(s), {} Cow(s)\n",
                    score.bulls, score.cows
                );

                //check that winning condition is satisfied
                if score.bulls == M {
                    println!(
                        "Congratulations !! You guessed the sequence {:?} in {} turns!",
                        target, turn
                    );
                    return true;
                }
                turn += 1;
            }
            
            //handling abort request
            Ok(None) => {
                println!(
                    "Game aborted. The secret target was: {:?}\n",
                    target
                );
                return false;
            }
            Err(err_msg) => {
                //print validation OR parse errors to user
                println!("Error: {}\n", err_msg);
            }
        }
    }
}

// overall program execution
fn main() {
    println!("==========================================");
    println!("        WELCOME TO MASTERMIND             ");
    println!("==========================================");

    loop {
        play_game(); //this runs a round

        print!("Would you like to play another round? (y/n): ");
        io::stdout().flush().unwrap();

        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();

        // terminate execution unless response is 'y' or 'Y'
        if !response.trim().eq_ignore_ascii_case("y") {
            println!("Thanks for playing! Goodbye.");
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //test scenario where all numbers are correct and in the right position
    #[test]
    fn test_all_bulls() {
        let target = vec![1, 2, 3, 4];
        let guess = vec![1, 2, 3, 4];
        assert_eq!(evaluate_guess(&target, &guess), Score { bulls: 4, cows: 0 });
    }

    // test scenario where guess seq is the target seq inverted
    #[test]
    fn test_all_cows() {
        let target = vec![1, 2, 3, 4];
        let guess = vec![4, 3, 2, 1];
        assert_eq!(evaluate_guess(&target, &guess), Score { bulls: 0, cows: 4 });
    }
    // handling repeated entries in either target or guess seq
    #[test]
    fn test_duplicates_handling() {
        let target = vec![1, 1, 2, 3];
        let guess = vec![1, 1, 1, 4];
        assert_eq!(evaluate_guess(&target, &guess), Score { bulls: 2, cows: 0 });

        let target = vec![2, 1, 3, 4];
        let guess = vec![1, 2, 1, 5];
        assert_eq!(evaluate_guess(&target, &guess), Score { bulls: 0, cows: 2 });
    }
}
