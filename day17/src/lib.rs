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

pub fn simulate(rocks: &[Rock], currents: &[Current], c: usize) -> Vec<Vec<u8>> {
    let mut rocks = rocks.iter().cycle();
    let mut currents = currents.iter().cycle();

    let mut cave = vec![];
    (0..c).for_each(|_| {
        let prev_max_height = cave.len() as isize;
        let mut max_x = 0;
        let mut rock: Rock = rocks
            .next()
            .unwrap()
            .iter()
            .map(|(x, y)| {
                let x = prev_max_height + x + 3;
                let y = y + 2;
                max_x = max_x.max(x);
                (x, y)
            })
            .collect();

        cave.resize(max_x as usize, vec![0; 7]);

        loop {
            let current = currents.next().unwrap();
            let (left, right) = rock
                .iter()
                .fold((7, 0), |acc, (_, y)| (acc.0.min(*y), acc.1.max(*y)));

            rock.iter_mut().for_each(|piece| {
                piece.0 += 1;
                piece.1 += match current {
                    Current::Left => (left > 0).then_some(-1),
                    Current::Right => (right - 1 > 7).then_some(1),
                }
                .unwrap_or(0);
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

        #[test]
        fn example_works() {
            let input = include_str!("../rocks");
            let rocks = parse_rocks(input).unwrap();
            let input = include_str!("../example");
            let currents = parse_currents(input).unwrap();

            let _world = simulate(&rocks, &currents, 2022);

            assert_eq!(0, 3068);
        }
    }
}
