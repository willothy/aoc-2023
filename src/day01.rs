const INPUT: &str = include_str!("../input/day01.txt");

pub mod part1 {

    pub fn solve() -> u64 {
        let mut total = 0;

        for line in super::INPUT.lines() {
            let mut first = None;
            let mut last = None;

            for c in line.chars() {
                if c.is_digit(10) {
                    match first {
                        None => first = Some(c),
                        Some(_) => last = Some(c),
                    }
                }
            }

            let first = first.unwrap();
            let last = last.unwrap_or(first);

            let number = match String::from_iter([first, last]).parse::<u64>() {
                Ok(n) => n,
                Err(_) => panic!("Failed to parse"),
            };

            total += number;
        }

        total
    }
}

pub mod part2 {
    use std::collections::HashMap;

    pub fn solve() -> u64 {
        let words: HashMap<&str, char> = HashMap::from_iter([
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ]);

        let mut total = 0;
        let mut buffer = String::new();

        for line in super::INPUT.lines() {
            let mut first = None;
            let mut last = None;
            let mut head_offset = 0;

            let mut next = |ch| match first {
                None => first = Some(ch),
                Some(_) => last = Some(ch),
            };

            for c in line.chars() {
                match c {
                    '0'..='9' => {
                        next(c);
                    }
                    'a'..='z' => {
                        buffer.push(c);
                        let mut substr = &mut buffer[head_offset..];

                        while substr.len() >= 3 {
                            match words.get(substr) {
                                Some(ch) => {
                                    next(*ch);
                                    head_offset += 1;
                                    break;
                                }
                                None => {
                                    substr = &mut substr[1..];
                                }
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }

            let first = first.unwrap();
            let last = last.unwrap_or(first);

            let number = match String::from_iter([first, last]).parse::<u64>() {
                Ok(n) => n,
                Err(_) => panic!("Failed to parse"),
            };

            total += number;
            buffer.clear();
        }

        total
    }
}
