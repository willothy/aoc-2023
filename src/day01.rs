pub mod part1 {
    const INPUT: &str = include_str!("../input/day01.txt");

    pub fn solve() -> u64 {
        let mut total = 0;

        for line in INPUT.lines() {
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
    //
}
