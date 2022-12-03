use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(PartialEq)]
enum RPC {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct UnknownCharError(char);

impl RPC {
    fn from_elf(c: &char) -> Result<Self, UnknownCharError> {
        match c {
            'A' => Ok(Self::Rock),
            'B' => Ok(Self::Paper),
            'C' => Ok(Self::Scissors),
            _ => Err(UnknownCharError(*c)),
        }
    }

    fn from_self(c: &char) -> Result<Self, UnknownCharError> {
        match c {
            'X' => Ok(Self::Rock),
            'Y' => Ok(Self::Paper),
            'Z' => Ok(Self::Scissors),
            _ => Err(UnknownCharError(*c)),
        }
    }

    fn winner(left: &Self, right: &Self) -> Winner {
        match left {
            left if left == right => Winner::Draw,
            RPC::Rock => {
                if right == &Self::Scissors {
                    Winner::Left
                } else {
                    Winner::Right
                }
            }
            RPC::Paper => {
                if right == &Self::Rock {
                    Winner::Left
                } else {
                    Winner::Right
                }
            }
            RPC::Scissors => {
                if right == &Self::Paper {
                    Winner::Left
                } else {
                    Winner::Right
                }
            }
        }
    }

    fn play(left: &Self, right: &Self) -> (usize, usize) {
        let winner = Self::winner(left, right);
        let left_score = Self::score(left);
        let right_score = Self::score(right);

        match winner {
            Winner::Draw => (3 + left_score, 3 + right_score),
            Winner::Left => (6 + left_score, right_score),
            Winner::Right => (left_score, 6 + right_score),
        }
    }

    fn score(s: &Self) -> usize {
        match s {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

enum Winner {
    Left,
    Right,
    Draw,
}

#[derive(PartialEq)]
enum MyOutcome {
    Win,
    Lose,
    Draw,
}

impl MyOutcome {
    fn parse(c: char) -> Result<Self, UnknownCharError> {
        match c {
            'X' => Ok(Self::Lose),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(UnknownCharError(c)),
        }
    }

    fn versus(self, other: &RPC) -> RPC {
        if self == Self::Draw {
            return match other {
                RPC::Rock => RPC::Rock,
                RPC::Paper => RPC::Paper,
                RPC::Scissors => RPC::Scissors,
            };
        }
        match other {
            RPC::Rock => {
                if self == Self::Win {
                    RPC::Paper
                } else {
                    RPC::Scissors
                }
            }
            RPC::Paper => {
                if self == Self::Win {
                    RPC::Scissors
                } else {
                    RPC::Rock
                }
            },
            RPC::Scissors => {
                if self == Self::Win {
                    RPC::Rock
                } else {
                    RPC::Paper
                }
            },
        }
    }
}

fn main() {
    let file = File::open("inputs/day2").unwrap();

    let (elf_score, my_score) =
        io::BufReader::new(file)
            .lines()
            .fold((0, 0), |(elf_score, my_score), line| {
                let chars = line.unwrap().chars().take(3).collect::<Vec<char>>();
                let elf = RPC::from_elf(chars.get(0).unwrap()).unwrap();
                let my_outcome = MyOutcome::parse(*chars.get(2).unwrap()).unwrap();
                let me = my_outcome.versus(&elf);
                let (elf_points, my_points) = RPC::play(&elf, &me);
                println!("{} - {}", elf_points, my_points);
                (elf_score + elf_points, my_score + my_points)
            });

    println!("{} - {}", elf_score, my_score);
}
