use std::cmp::Ordering;
mod input;

fn main() {
    let mut game_count = 0;

    loop {
        game_count += 1;

        let from = input::ask_user_a_number("From:", None, None);
        let to = input::ask_user_a_number("To:", Some(from), None);

        let num = rand::random_range(from..=to);

        println!("Guess the number between {from} and {to}");

        let mut tries = 0;

        loop {
            tries += 1;

            let guess = input::ask_user_a_number("Type your guess:", Some(from), Some(to));

            match guess.cmp(&num) {
                Ordering::Less => println!("Sorry, but you're looking for a bigger number than: {guess}"),
                Ordering::Greater => println!("Sorry, but you're looking for a smaller number than: {guess}"),
                Ordering::Equal => {
                    let try_str = if tries > 1 {
                        "tries"
                    } else {
                        "try"
                    };

                    println!("Great job!");
                    println!("You found the number. It was: {num}");
                    println!("And it only took you {tries} {try_str}");

                    break;
                },
            }
        }

        if input::prompt_line("Do you want to play again? (y/n):").to_lowercase() != "y" {
            break;
        }
    }

    let game_str = if game_count > 1 {
        "games"
    } else {
        "game"
    };

    println!("\nThanks for playing!");
    println!("You played {game_count} {game_str}.");
}