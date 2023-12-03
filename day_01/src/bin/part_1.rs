fn main() {
    let day_1_document = include_str!("input.txt");
    let calibration_sum = calibration_document_sum(&day_1_document);
    println!("{calibration_sum}"); // 55488
}

fn calibration_document_sum(document: &str) -> usize {
    document
        .lines()
        .map(|line| string_to_calibration_value(line))
        .sum()
}

fn string_to_calibration_value(s: &str) -> usize {
    let digits: Vec<char> = s.chars().filter(|c| c.is_numeric()).collect();
    let first = digits[0];
    let last = digits[digits.len() - 1];

    format!("{first}{last}").parse().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{calibration_document_sum, string_to_calibration_value};

    #[test]
    fn string_to_number_1() {
        let result = string_to_calibration_value("1abc2");
        assert_eq!(result, 12);
    }

    #[test]
    fn string_to_number_2() {
        let result = string_to_calibration_value("pqr3stu8vwx");
        assert_eq!(result, 38);
    }

    #[test]
    fn string_to_number_3() {
        let result = string_to_calibration_value("a1b2c3d4e5f");
        assert_eq!(result, 15);
    }

    #[test]
    fn string_to_number_4() {
        let result = string_to_calibration_value("treb7uchet");
        assert_eq!(result, 77);
    }

    // two1nine 29
    // eightwothree 83
    // abcone2threexyz 13
    // xtwone3four 24
    // 4nineeightseven2 42
    // zoneight234 14
    // 7pqrstsixteen 76

    // current character and next character

    #[test]
    fn string_to_number_5() {
        let result = string_to_calibration_value("two1nine");
        assert_eq!(result, 29);
    }

    #[test]
    fn string_to_number_6() {
        let result = string_to_calibration_value("eightwothree");
        assert_eq!(result, 83);
    }

    #[test]
    fn string_to_number_7() {
        let result = string_to_calibration_value("abcone2threexyz");
        assert_eq!(result, 13);
    }

    #[test]
    fn string_to_number_7() {
        let result = string_to_calibration_value("xtwone3four");
        assert_eq!(result, 24);
    }

    #[test]
    fn string_to_number_7() {
        let result = string_to_calibration_value("4nineeightseven2");
        assert_eq!(result, 42);
    }

    #[test]
    fn string_to_number_7() {
        let result = string_to_calibration_value("zoneight234");
        assert_eq!(result, 14);
    }

    #[test]
    fn string_to_number_7() {
        let result = string_to_calibration_value("7pqrstsixteen");
        assert_eq!(result, 76);
    }

    #[test]
    fn test_calibration_document_sum() {
        let document = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = calibration_document_sum(document);
        assert_eq!(result, 142);
    }
}
