use std::fs::File;
use std::io::{BufRead, BufReader};

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

    for c in line.chars() {
        if c.is_digit(10) {
            digits.push(c.to_string());
        }
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
            let calibration_value = calc_calibration_value(&line);
            sum += calibration_value as i32;
        }
    }

    println!("Calibration value: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_digits() {
        assert_eq!(find_digits("abc123def456"), vec!["1", "2", "3", "4", "5", "6"]);
        assert_eq!(find_digits("no digits here"), vec![] as Vec<String>);
        assert_eq!(find_digits("111"), vec!["1", "1", "1"]);
    }

    #[test]
    fn test_calc_calibration_value() {
        assert_eq!(calc_calibration_value("abc123def456"), 16);
        assert_eq!(calc_calibration_value("no digits here"), 0);
        assert_eq!(calc_calibration_value("111"), 11);
        assert_eq!(calc_calibration_value("2"), 22);
    }
}