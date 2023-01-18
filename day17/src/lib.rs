use std::fmt::Display;

use nom::{
    character::complete::{newline, one_of},
    multi::{count, many1, separated_list1},
    IResult,
};

type Rock = Vec<(isize, isize)>;

fn parse_rock(input: &str) -> IResult<&str, Rock> {
    separated_list1(newline, many1(one_of(".#")))(input).map(|(input, rock)| {
        (
            input,
            rock.iter()
                .enumerate()
                .flat_map(|(x, line)| {
                    let x = (rock.len() - x) as isize;
                    line.iter()
                        .enumerate()
                        .filter(|(_, c)| c == &&'#')
                        .map(move |(y, _)| (x, y as isize))
                })
                .collect(),
        )
    })
}

pub fn parse_rocks(input: &str) -> Result<Vec<Rock>, String> {
    let (_, rocks) =
        separated_list1(count(newline, 2), parse_rock)(input).map_err(|e| e.to_string())?;

    Ok(rocks)
}

pub enum Current {
    Left = -1,
    Right = 1,
}

impl Display for Current {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Current::Left => "<",
            Current::Right => ">",
        })
    }
}

pub fn parse_currents(input: &str) -> Result<Vec<Current>, String> {
    input
        .trim_end()
        .chars()
        .map(|c| match c {
            '<' => Ok(Current::Left),
            '>' => Ok(Current::Right),
            _ => Err("Other char found in str.".to_string()),
        })
        .collect()
}

pub fn simulate(rocks: &[Rock], currents: &[Current], c: usize) -> Vec<Vec<u8>> {
    let mut rocks = rocks.iter().cycle();
    let mut currents = currents.iter().cycle();

    let t = vec![0; 7];
    let mut cave: Vec<Vec<u8>> = vec![];
    (0..c).for_each(|c| {
        if c % 1_000_000 == 0 {
            println!("now dropping rock number {c}");
        }
        let prev_max_height = cave
            .iter()
            .enumerate()
            .rev()
            .find_map(|(x, line)| line.iter().any(|c| c == &1).then_some(x + 1))
            .unwrap_or(0) as isize;

        let mut max_x = 0;
        let mut rock: Rock = rocks
            .next()
            .unwrap()
            .iter()
            .map(|(x, y)| {
                let x = prev_max_height + x + 2;
                let y = y + 2;
                max_x = max_x.max(x);
                (x, y)
            })
            .collect();
        // println!("{:?}", rock);

        cave.resize(max_x as usize, t.clone());

        for _ in 0.. {
            let current = currents.next().unwrap();
            // println!("Loop {}, wind pushing to {}", i, current);
            let (left, right) = rock
                .iter()
                .fold((7, 0), |acc, (_, y)| (acc.0.min(*y), acc.1.max(*y)));
            // println!("left: {left}, right: {right}");

            // wind push
            let rock2: Rock = rock
                .iter()
                .map(|(x, y)| {
                    (
                        *x,
                        *y + match current {
                            Current::Left => (left > 0).then_some(-1),
                            Current::Right => (right + 1 < 7).then_some(1),
                        }
                        .unwrap_or(0),
                    )
                })
                .collect();

            let rock2: Rock = if is_colliding(&rock2, cave.as_slice()) {
                rock
            } else {
                rock2
            };
            // println!("\nAfter wind");
            // print_rock(&rock2);

            // drop one
            let rock3: Rock = rock2.iter().map(|(x, y)| (*x - 1, *y)).collect();
            // println!("\nAfter gravity");
            // print_rock(&rock3);
            if is_colliding(&rock3, cave.as_slice()) {
                settle_rock(&rock2, &mut cave);
                break;
            } else {
                rock = rock3;
            }
        }

        // println!(" ------- ");
        // for line in cave.iter().rev() {
        //     println!(
        //         "|{}|",
        //         line.iter()
        //             .map(|n| match n {
        //                 1 => '#',
        //                 _ => ' ',
        //             })
        //             .collect::<String>()
        //     );
        // }
        // println!(" ------- ");
        // pause();
    });

    cave
}

fn settle_rock(rock: &Rock, cave: &mut [Vec<u8>]) {
    rock.iter().for_each(|(x, y)| {
        cave[*x as usize][*y as usize] = 1;
    });
}

fn is_colliding(rock: &Rock, cave: &[Vec<u8>]) -> bool {
    let (min_x, max_x) = rock
        .iter()
        .map(|(x, _)| (x, x))
        .reduce(|(min_x, max_x), (x, _)| (min_x.min(x), max_x.max(x)))
        .unwrap();
    if min_x == &-1 {
        return true;
    }
    let min_x = *min_x as usize;
    let max_x = *max_x as usize;
    cave.iter()
        .enumerate()
        .rev()
        .filter(|(x, _)| &min_x <= x && x <= &max_x)
        .any(|(x, row)| {
            let x = x as isize;
            row.iter()
                .enumerate()
                .filter(|(_, c)| c == &&1)
                .any(|(y, _)| {
                    let y = y as isize;
                    rock.iter().any(|(rx, ry)| (rx, ry) == (&x, &y))
                })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_parser() {
        let input = include_str!("../rocks");
        let rocks = parse_rocks(input).unwrap();
        for rock in rocks.iter() {
            println!("{:?}", rock);
        }
        assert_eq!(rocks.len(), 5);
    }

    #[test]
    fn current_parser() {
        let input = include_str!("../example");
        let currents = parse_currents(input).unwrap();
        assert_eq!(currents.len(), 40);
        assert_eq!(
            currents
                .into_iter()
                .map(|d| d as isize)
                .collect::<Vec<isize>>(),
            &[
                1, 1, 1, -1, -1, 1, -1, 1, 1, -1, -1, -1, 1, 1, -1, 1, 1, 1, -1, -1, -1, 1, 1, 1,
                -1, -1, -1, 1, -1, -1, -1, 1, 1, -1, 1, 1, -1, -1, 1, 1
            ]
        );
    }

    mod part1 {
        use super::*;

        fn test_helper(rocks: &[Rock], currents: &[Current], iterations: usize) -> usize {
            let world = simulate(rocks, currents, iterations);

            world
                .iter()
                .enumerate()
                .rev()
                .find_map(|(x, line)| line.iter().any(|c| c == &1).then_some(x + 1))
                .unwrap_or(0)
        }

        #[test]
        fn example_works() {
            let input = include_str!("../rocks");
            let rocks = parse_rocks(input).unwrap();
            let input = include_str!("../example");
            let currents = parse_currents(input).unwrap();

            assert_eq!(test_helper(&rocks, &currents, 2022), 3068);
        }

        #[test]
        fn input_works() {
            let input = include_str!("../rocks");
            let rocks = parse_rocks(input).unwrap();
            let input = include_str!("../input");
            let currents = parse_currents(input).unwrap();

            assert_eq!(test_helper(&rocks, &currents, 2022), 3166);
        }
    }

    mod part2 {
        use super::*;
        fn test_helper(rocks: &[Rock], currents: &[Current], iterations: usize) -> usize {
            let world = simulate(rocks, currents, iterations);

            world
                .iter()
                .enumerate()
                .rev()
                .find_map(|(x, line)| line.iter().any(|c| c == &1).then_some(x + 1))
                .unwrap_or(0)
        }

        #[test]
        fn example_works() {
            let input = include_str!("../rocks");
            let rocks = parse_rocks(input).unwrap();
            let input = include_str!("../example");
            let currents = parse_currents(input).unwrap();

            assert_eq!(test_helper(&rocks, &currents, 1_000_000_000_000), 1_514_285_714_288);
        }

        #[test]
        #[ignore]
        fn input_works() {
            let input = include_str!("../rocks");
            let rocks = parse_rocks(input).unwrap();
            let input = include_str!("../input");
            let currents = parse_currents(input).unwrap();

            assert_eq!(test_helper(&rocks, &currents, 2022), 3166);
        }
    }
}
