const INPUT: &str = include_str!("../input/day02.txt");

const RED: u64 = 12;
const GREEN: u64 = 13;
const BLUE: u64 = 14;

#[derive(Debug)]
struct Game {
    id: u64,
    r: u64,
    g: u64,
    b: u64,
}

impl Game {
    pub fn parse(str: impl AsRef<str>) -> Self {
        // The line always starts with "Game "
        let str = &mut str.as_ref()[4..].split(":");
        let id = str.next().unwrap().trim().parse::<u64>().unwrap();

        let mut rounds = str.next().unwrap().trim().split(";");

        let mut game = Game {
            id,
            r: 0,
            g: 0,
            b: 0,
        };

        while let Some(round) = rounds.next() {
            let mut scores = round.trim().split(",");
            while let Some(score) = scores.next() {
                let mut chunk = score.trim().split(" ");

                let count = chunk.next().unwrap().trim().parse::<u64>().unwrap();
                let color = chunk.next().unwrap();

                match color {
                    "red" => game.r = game.r.max(count),
                    "green" => game.g = game.g.max(count),
                    "blue" => game.b = game.b.max(count),
                    _ => unreachable!(),
                }
            }
        }

        game
    }

    pub fn is_valid(&self) -> bool {
        self.r <= RED && self.g <= GREEN && self.b <= BLUE
    }
}

pub mod part1 {
    use super::Game;

    pub fn solve() -> u64 {
        super::INPUT
            .lines()
            .map(Game::parse)
            .filter(Game::is_valid)
            .map(|game| game.id)
            .sum()
    }
}

pub mod part2 {
    use super::Game;

    pub fn solve() -> u64 {
        super::INPUT
            .lines()
            .map(Game::parse)
            .map(|game| game.r * game.g * game.b)
            .sum()
    }
}
