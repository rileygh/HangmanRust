use std::io;

fn main() {
    let secret_word = "hello";
    let mut word_progress = Vec::new();
    for _ in 0..secret_word.len() {
        word_progress.push('_');
    }
    let mut remaining_guesses = 5;

    while remaining_guesses > 0 {
        println!("Word progress: {:?}", word_progress);
        println!("Remaining guesses: {}", remaining_guesses);

        println!("Enter a letter:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim().to_lowercase();

        if guess.len() != 1 || !guess.chars().next().unwrap().is_alphabetic() {
            println!("Please enter a single letter.");
            continue;
        }

        let guess = guess.chars().next().unwrap();
        let mut correct_guess = false;
        for (i, c) in secret_word.chars().enumerate() {
            if c == guess {
                word_progress[i] = c;
                correct_guess = true;
            }
        }

        if !correct_guess {
            remaining_guesses -= 1;
        }

        if !word_progress.contains(&'_') {
            println!("You win! The word was: {}", secret_word);
            break;
        }
    }

    if remaining_guesses == 0 {
        println!("You lose! The word was: {}", secret_word);
    }
}