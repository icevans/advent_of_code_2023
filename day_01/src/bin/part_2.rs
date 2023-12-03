use std::time::Instant;
use day_01::digit_words;

fn main() {
    let day_1_document = include_str!("input.txt");
    let start = Instant::now();
    let calibration_sum = calibration_document_sum(&day_1_document);
    let duration = start.elapsed();
    println!("{calibration_sum}");
    println!("{:?}", duration);
}

fn calibration_document_sum(document: &str) -> usize {
    document
        .lines()
        .map(|line| string_to_calibration_value(line))
        .sum()
}

fn string_to_calibration_value(s: &str) -> usize {
    let mut digits: Vec<usize> = Vec::new();

    for (i, c) in s.char_indices() {
        if c.is_numeric() {
            digits.push(c.to_string().parse().unwrap());
            continue;
        }

        if let Some(digit_word) = digit_words::find_digit_word_starting_at_position(i, &s) {
            digits.push(digit_words::DIGIT_WORDS[digit_word])
        }
    }

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
    fn string_to_number_8() {
        let result = string_to_calibration_value("xtwone3four");
        assert_eq!(result, 24);
    }

    #[test]
    fn string_to_number_9() {
        let result = string_to_calibration_value("4nineeightseven2");
        assert_eq!(result, 42);
    }

    #[test]
    fn string_to_number_10() {
        let result = string_to_calibration_value("zoneight234");
        assert_eq!(result, 14);
    }

    #[test]
    fn string_to_number_11() {
        let result = string_to_calibration_value("7pqrstsixteen");
        assert_eq!(result, 76);
    }

    #[test]
    fn string_to_number_12() {
        let result = string_to_calibration_value("eightwo");
        assert_eq!(result, 82);
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
