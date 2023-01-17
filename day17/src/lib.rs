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

pub fn simulate(rocks: &Vec<Rock>, currents: &Vec<Current>, c: usize) -> Vec<Vec<u8>> {
    let mut rocks = rocks.iter().cycle();
    let mut currents = currents.iter().cycle();

    let mut cave = vec![];
    (0..c).for_each(|i| {
        let prev_max_height = cave.len() as isize;
        let mut rock: Rock = rocks
            .next()
            .unwrap()
            .iter()
            .map(|(x, y)| (prev_max_height + x + 3, y + 2))
            .collect();
        println!("{:?}", rock);
        loop {
            let current = currents.next().unwrap();
            let (left, right) = rock
                .iter()
                .fold((7, 0), |acc, (_, y)| (acc.0.min(*y), acc.1.max(*y)));

            rock.iter_mut().for_each(|piece| {
                piece.0 += 1;
                piece.1 += match current {
                    Current::Left => {
                        if left > 0 {
                            -1
                        } else {
                            0
                        }
                    }
                    Current::Right => {
                        if right - 1 > 7 {
                            1
                        } else {
                            0
                        }
                    }
                };
            });
        }
    });

    cave
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
            &currents,
            &[
                1, 1, 1, -1, -1, 1, -1, 1, 1, -1, -1, -1, 1, 1, -1, 1, 1, 1, -1, -1, -1, 1, 1, 1,
                -1, -1, -1, 1, -1, -1, -1, 1, 1, -1, 1, 1, -1, -1, 1, 1
            ]
        );
    }

    mod part1 {
        use super::*;

        #[test]
        fn example_works() {
            let input = include_str!("../rocks");
            let rocks = parse_rocks(input).unwrap();
            let input = include_str!("../example");
            let currents = parse_currents(input).unwrap();

            let world = simulate(&rocks, &currents, 2022);

            assert_eq!(world.iter().fold(0, |acc, rock| acc.max(rock[0].0)), 3068);
        }
    }
}
