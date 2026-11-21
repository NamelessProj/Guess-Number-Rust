use std::io;

/// Prompt the user to enter a number with the posibility to add a minimum and a maximum value.
/// 
/// # Arguments
/// * `question` - What to ask the user for.
/// * `min` - The smallest number authorized to be entered by the user.
/// * `max` - The biggest number authorized to be entered by the user.
/// 
/// # Returns
/// A number `i32`
/// 
/// # Panics
/// Panics if the minimum value is set to be bigger or equal than the maximum value.
fn ask_user_a_number(question: &str, min: Option<i32>, max: Option<i32>) -> i32 {
    let mut input_str = String::new();
    let mut output: i32;

    if min.is_some() && max.is_some() && min.unwrap() >= max.unwrap() {
        panic!("The minimum cannot be bigger than the maximum!");
    }

    loop {
        input_str.clear();

        println!();
        println!("{question}");
        io::stdin().read_line(&mut input_str).expect("Failed to read the line");

        output = match input_str.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("That's not a valid number, try again.");
                continue;
            }
        };

        if min.is_some() && output < min.unwrap() {
            println!("We need a number bigger than: {}", min.unwrap());
            continue;
        }

        if max.is_some() && output > max.unwrap() {
            println!("We need a number smaller than: {}", max.unwrap());
            continue;
        }

        break;
    }

    output
}

fn main() {
    let from = ask_user_a_number("From:", None, None);
    let to = ask_user_a_number("To:", Some(from), None);

    let num = rand::random_range(from..=to);

    println!("Guess the number between {from} and {to}");

    let mut _tries = 0;

    loop {
        _tries += 1;

        let guess = ask_user_a_number("Type your guess:", Some(from), Some(to));

        if guess < num {
            println!("Sorry, but you're looking for a bigger number than: {guess}");
        } else if guess > num {
            println!("Sorry, but you're looking for a smaller number than: {guess}");
        } else {
            let try_str = if _tries > 1 {
                "tries"
            } else {
                "try"
            };

            println!("Great job!");
            println!("You found the number. It was: {num}");
            println!("And it only took you {_tries} {try_str}");

            break;
        }
    }
}