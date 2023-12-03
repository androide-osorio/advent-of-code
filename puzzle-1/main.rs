use fancy_regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn parse_digit_text(text: &str) -> Option<&str> {
    let mut digit_map = HashMap::new();

    digit_map.insert("one", "1");
    digit_map.insert("two", "2");
    digit_map.insert("three", "3");
    digit_map.insert("four", "4");
    digit_map.insert("five", "5");
    digit_map.insert("six", "6");
    digit_map.insert("seven", "7");
    digit_map.insert("eight", "8");
    digit_map.insert("nine", "9");
    digit_map.insert("zero", "0");

    if text.chars().all(char::is_numeric) {
        return Some(text);
    }

    digit_map.get(text).map(|&x| x)
}

/// Finds and returns a vector of digits from the given string.
///
/// # Arguments
///
/// * `line` - The input string from which to find the digits.
///
/// # Returns
///
/// A vector containing the digits found in the input string.
fn find_digits(line: &str) -> Vec<String> {
    let mut digits = Vec::new();
    let re = Regex::new(r#"(?=(\d|one|two|three|four|five|six|seven|eight|nine|zero))"#).unwrap();

    for result in re.captures_iter(line) {
        let captures = result.expect("Error running regex");
        let digit = captures.get(1).unwrap().as_str();
        let parsed_digit = parse_digit_text(digit).unwrap();
        digits.push(parsed_digit.to_string());
    }

    digits
}

/// Calculates the calibration value based on the given line.
///
/// # Arguments
///
/// * `line` - A string slice representing the line to calculate the calibration value from.
///
/// # Returns
///
/// The calculated calibration value as an unsigned 8-bit integer.
fn calc_calibration_value(line: &str) -> u8 {
    let mut calibration_value: u8 = 0;
    let digits = find_digits(line);

    if digits.len() == 0 {
        return calibration_value;
    }

    if digits.len() == 1 {
        calibration_value = digits[0].repeat(2).parse::<u8>().unwrap();
    } else {
        let first_digit = &digits[0];
        let last_digit = &digits[digits.len() - 1];
        let calibration_str = format!("{first_digit}{last_digit}");
        calibration_value = calibration_str.parse::<u8>().unwrap();
    }
    calibration_value
}

fn main() {
    let file = File::open("calibration-values.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut sum: i32 = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let digits = find_digits(&line);
            let calibration_value = calc_calibration_value(&line);

            println!("{} <- {:?} <- {}", calibration_value, digits.join(""), line);
            sum += calibration_value as i32;
        }
    }

    println!("Final calibration value: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_digit_text() {
        assert_eq!(parse_digit_text("1"), Some("1"));
        assert_eq!(parse_digit_text("1234"), Some("1234"));
        assert_eq!(parse_digit_text("one"), Some("1"));
        assert_eq!(parse_digit_text("two"), Some("2"));
        assert_eq!(parse_digit_text("notanumber"), None);
    }

    #[test]
    fn test_find_digits() {
        assert_eq!(find_digits("abc123def456"), vec!["1", "2", "3", "4", "5", "6"]);
        assert_eq!(find_digits("no digits here"), vec![] as Vec<String>);
        assert_eq!(find_digits("111"), vec!["1", "1", "1"]);
        assert_eq!(find_digits("2"), vec!["2"]);
        assert_eq!(find_digits("fourfive6seven"), vec!["4", "5", "6", "7"]);
        assert_eq!(find_digits("zoneight234"), vec!["1", "8", "2", "3", "4"]);
    }

    #[test]
    fn test_calc_calibration_value() {
        assert_eq!(calc_calibration_value("abc123def456"), 16);
        assert_eq!(calc_calibration_value("no digits here"), 0);
        assert_eq!(calc_calibration_value("111"), 11);
        assert_eq!(calc_calibration_value("2"), 22);
        assert_eq!(calc_calibration_value("abc1pqsttwo33four"), 14);
        assert_eq!(calc_calibration_value("zoneight234"), 14);
    }
}