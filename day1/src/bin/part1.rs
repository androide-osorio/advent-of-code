
fn find_digits(line: &str) -> Vec<String> {
    line.chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
}

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
    let data = include_str!("../data.txt");

    let sum = data
        .lines()
        .map(|line| {
            let digits = find_digits(&line);
            let calibration_value = calc_calibration_value(&line);
            println!("{} <- {:?} <- {}", calibration_value, digits.join(""), line);
            calibration_value
        })
        .sum::<u8>();

    println!("Final calibration value: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_digits() {
        assert_eq!(
            find_digits("abc123def456"),
            vec!["1", "2", "3", "4", "5", "6"]
        );
        assert_eq!(find_digits("no digits here"), vec![] as Vec<String>);
        assert_eq!(find_digits("111"), vec!["1", "1", "1"]);
        assert_eq!(find_digits("2"), vec!["2"]);
        assert_eq!(find_digits("fourfive6seven"), vec!["6"]);
        assert_eq!(find_digits("zoneight234"), vec!["2", "3", "4"]);
    }

    #[test]
    fn test_calc_calibration_value() {
        assert_eq!(calc_calibration_value("abc123def456"), 16);
        assert_eq!(calc_calibration_value("no digits here"), 0);
        assert_eq!(calc_calibration_value("111"), 11);
        assert_eq!(calc_calibration_value("2"), 22);
        assert_eq!(calc_calibration_value("abc1pqsttwo33four"), 13);
        assert_eq!(calc_calibration_value("zoneight234"), 24);
    }
}
