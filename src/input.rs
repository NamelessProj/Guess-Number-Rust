use std::io;

/// Prompt the user to enter a line of text.
/// 
/// # Arguments
/// * `question` - What to ask the user for.
/// 
/// # Returns
/// The line entered by the user.
pub fn prompt_line(question: &str) -> String {
    let mut input = String::new();

    input.clear();

    println!("\n{question}");
    io::stdin().read_line(&mut input).expect("Failed to read the line");

    input.trim().to_string()
}

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

    loop {
        let input = prompt_line(question);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_valid_number() {
        assert_eq!(parse_in_range("42\n", None, None), Ok(42));
    }

    #[test]
    fn accepts_negative_numbers() {
        assert_eq!(parse_in_range("-7", None, None), Ok(-7));
    }

    #[test]
    fn rejects_non_numbers() {
        assert!(parse_in_range("abc", None, None).is_err());
        assert!(parse_in_range("", None, None).is_err());
        assert!(parse_in_range("3.5", None, None).is_err());
        assert!(parse_in_range("3,5", None, None).is_err());
    }

    #[test]
    fn rejects_below_minimum() {
        assert!(parse_in_range("5", Some(10), None).is_err());
    }

    #[test]
    fn rejects_above_maximum() {
        assert!(parse_in_range("15", None, Some(10)).is_err());
    }

    #[test]
    fn accepts_at_boundaries() {
        assert_eq!(parse_in_range("5", Some(5), Some(10)), Ok(5));
        assert_eq!(parse_in_range("10", Some(5), Some(10)), Ok(10));
    }

    #[test]
    #[should_panic(expected = "The minimum value must be smaller than the maximum value.")]
    fn panics_if_minimum_is_greater_than_maximum() {
        ask_user_a_number("Enter a number:", Some(10), Some(5));
    }
}