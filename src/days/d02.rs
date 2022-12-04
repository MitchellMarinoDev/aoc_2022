// https://adventofcode.com/2022/day/2

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Game {
    /// Shape 1 : Our shape
    s1: Shape,
    /// Shape 2 : Opponent shape
    s2: Shape,
}

pub fn solve(input: String) -> (String, String) {
    let lines = input.lines();
    let mut games_p1 = Vec::with_capacity(lines.size_hint().0);
    let mut games_p2 = Vec::with_capacity(lines.size_hint().0);

    for line in lines {
        let mut split = line.split(' ');
        let s2 = split.next().expect("Missing player 2");
        let s1 = split.next().expect("Missing player 1");
        assert!(split.next().is_none(), "Too many players");

        let s2 = match s2 {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("unexpected shape {}", s2),
        };

        let p1_s1 = match s1 {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!("unexpected shape {}", s1),
        };

        let p2_s1 = match s1 {
            // we need to lose
            "X" => s2.loses_to(),
            // we need to draw
            "Y" => s2.draws(),
            // we need to win
            "Z" => s2.beats(),
            _ => panic!("unexpected shape {}", s1),
        };

        games_p1.push(Game {
            s1: p1_s1,
            s2,
        });

        games_p2.push(Game {
            s1: p2_s1,
            s2,
        });
    }

    let p1 = games_p1.iter().map(|g| g.score()).sum::<i32>();
    let p2 = games_p2.iter().map(|g| g.score()).sum::<i32>();

    (p1.to_string(), p2.to_string())
}

impl Game {
    fn score(&self) -> i32 {
        self.s1 as i32 + self.outcome() as i32
    }

    fn outcome(&self) -> Outcome {
        use Shape::*;

        match (self.s1, self.s2) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Outcome::Loss,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Outcome::Draw,
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Outcome::Win,
        }
    }
}

impl Shape {
    fn beats(&self) -> Shape {
        match self {
            Shape::Scissors => Shape::Rock,
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
        }
    }

    fn loses_to(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn draws(&self) -> Shape {
        *self
    }
}
