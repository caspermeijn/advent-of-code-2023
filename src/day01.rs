pub mod part1 {

    pub fn parse(text: &str) -> u32 {
        text.lines().map(parse_line).sum()
    }

    fn parse_line(text: &str) -> u32 {
        let numbers = text.chars().filter(|c| c.is_numeric());
        let first = numbers.clone().next().unwrap();
        let last = numbers.last().unwrap();
        let number = format!("{first}{last}");
        number.parse().unwrap()
    }

    #[cfg(test)]
    mod tests {
        use crate::day01::part1::{parse, parse_line};

        #[test]
        fn example() {
            assert_eq!(parse_line("1abc2"), 12);
            assert_eq!(parse_line("pqr3stu8vwx"), 38);
            assert_eq!(parse_line("a1b2c3d4e5f"), 15);
            assert_eq!(parse_line("treb7uchet"), 77);
        }

        #[test]
        fn input() {
            assert_eq!(parse(include_str!("../input/day01.txt")), 54159);
        }
    }
}

pub mod part2 {

    pub fn parse(text: &str) -> u32 {
        text.lines().map(parse_line).sum()
    }

    fn parse_line(text: &str) -> u32 {
        let mut chars = text.chars();
        let mut first_digit = None;
        let mut last_digit = None;
        loop {
            let digit = starts_with_digit(chars.as_str());
            first_digit = first_digit.or(digit);
            last_digit = digit.or(last_digit);
            if chars.next().is_none() {
                break;
            }
        }
        let first = first_digit.unwrap();
        let last = last_digit.unwrap();
        let number = format!("{first}{last}");
        number.parse().unwrap()
    }

    fn starts_with_digit(text: &str) -> Option<char> {
        if text.starts_with("one") {
            return Some('1');
        }
        if text.starts_with("two") {
            return Some('2');
        }
        if text.starts_with("three") {
            return Some('3');
        }
        if text.starts_with("four") {
            return Some('4');
        }
        if text.starts_with("five") {
            return Some('5');
        }
        if text.starts_with("six") {
            return Some('6');
        }
        if text.starts_with("seven") {
            return Some('7');
        }
        if text.starts_with("eight") {
            return Some('8');
        }
        if text.starts_with("nine") {
            return Some('9');
        }
        if let Some(first_char) = text.chars().next() {
            if first_char.is_numeric() {
                return Some(first_char);
            }
        }
        return None;
    }

    #[cfg(test)]
    mod tests {
        use crate::day01::part2::{parse, parse_line};

        #[test]
        fn example() {
            assert_eq!(parse_line("two1nine"), 29);
            assert_eq!(parse_line("eightwothree"), 83);
            assert_eq!(parse_line("abcone2threexyz"), 13);
            assert_eq!(parse_line("xtwone3four"), 24);
            assert_eq!(parse_line("4nineeightseven2"), 42);
            assert_eq!(parse_line("zoneight234"), 14);
            assert_eq!(parse_line("7pqrstsixteen"), 76);
        }

        #[test]
        fn input() {
            assert_eq!(parse(include_str!("../input/day01.txt")), 53866);
        }
    }
}
