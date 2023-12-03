
    // TODO: Can this actual just be an enum?
    pub static DIGIT_WORDS: phf::Map<&str, usize> = phf::phf_map! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
    };

    pub fn find_digit_word_starting_at_position(i: usize, haystack: &str) -> Option<&str> {
        let mut candidate: &str;

        for j in i..haystack.len() {
            candidate = &haystack[i..=j];

            if DIGIT_WORDS.contains_key(candidate) {
                return Some(candidate);
            }
        }

        None
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn happy_case() {
            let result_0 = find_digit_word_starting_at_position(0, "zero");
            let result_1 = find_digit_word_starting_at_position(0, "one");
            let result_2 = find_digit_word_starting_at_position(0, "two");
            let result_3 = find_digit_word_starting_at_position(0, "three");
            let result_4 = find_digit_word_starting_at_position(0, "four");
            let result_5 = find_digit_word_starting_at_position(0, "five");
            let result_6 = find_digit_word_starting_at_position(0, "six");
            let result_7 = find_digit_word_starting_at_position(0, "seven");
            let result_8 = find_digit_word_starting_at_position(0, "eight");
            let result_9 = find_digit_word_starting_at_position(0, "nine");

            assert_eq!(Some("zero"), result_0);
            assert_eq!(Some("one"), result_1);
            assert_eq!(Some("two"), result_2);
            assert_eq!(Some("three"), result_3);
            assert_eq!(Some("four"), result_4);
            assert_eq!(Some("five"), result_5);
            assert_eq!(Some("six"), result_6);
            assert_eq!(Some("seven"), result_7);
            assert_eq!(Some("eight"), result_8);
            assert_eq!(Some("nine"), result_9);
        }

        #[test]
        fn happy_case_3() {
            let result = find_digit_word_starting_at_position(0, "eightwothree");
            assert_eq!(Some("eight"), result)
        }

        #[test]
        fn happy_case_4() {
            let result = find_digit_word_starting_at_position(4, "eightwothree");
            assert_eq!(Some("two"), result)
        }

        #[test]
        fn happy_case_5() {
            let result = find_digit_word_starting_at_position(7, "eightwothree");
            assert_eq!(Some("three"), result)
        }

        #[test]
        fn sad_case() {
            let result = find_digit_word_starting_at_position(1, "one");
            assert_eq!(None, result);
        }

        #[test]
        fn sad_case_2() {
            let result = find_digit_word_starting_at_position(0, "o1ne");
            assert_eq!(None, result);
        }

        #[test]
        fn sad_case_3() {
            let result = find_digit_word_starting_at_position(0, "1one");
            assert_eq!(None, result);
        }
    }
