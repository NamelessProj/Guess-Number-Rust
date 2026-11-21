use std::io;

/// Prompt the user to enter a number with the possibility to add a minimum and a maximum value.
/// 
/// # Arguments
/// * `question` - What to ask the user for.
/// * `min` - The smallest number authorized to be entered by the user.
/// * `max` - The biggest number authorized to be entered by the user.
/// 
/// # Returns
/// The number entered by the user.
pub fn ask_user_a_number(question: &str, min: Option<i32>, max: Option<i32>) -> i32 {
    if let (Some(low), Some(high)) = (min, max) {
        if low > high {
            panic!("The minimum value must be smaller than the maximum value.");
        }
    }

    let mut input = String::new();
    loop {
        input.clear();

        println!();
        println!("{question}");
        io::stdin().read_line(&mut input).expect("Failed to read the line");

        match parse_in_range(&input, min, max) {
            Ok(n) => return n,
            Err(s) => println!("{s}"),
        }
    }
}

/// Parse a string to an integer and check if it is in the range of the minimum and maximum values.
/// 
/// # Arguments
/// * `input` - The string to parse.
/// * `min` - The smallest number authorized to be entered by the user.
/// * `max` - The biggest number authorized to be entered by the user.
/// 
/// # Returns
/// A result containing the parsed number or an error message.
fn parse_in_range(input: &str, min: Option<i32>, max: Option<i32>) -> Result<i32, String> {
    let n: i32 = input.trim().parse::<i32>().map_err(|_| "That's not a valid number, try again.".to_string())?;

    if let Some(low) = min {
        if low > n {
            return Err(format!("We need a number bigger than or equal to: {low}"));
        }
    }

    if let Some(high) = max {
        if high < n {
            return Err(format!("We need a number smaller than or equal to: {high}"));
        }
    }

    Ok(n)
}