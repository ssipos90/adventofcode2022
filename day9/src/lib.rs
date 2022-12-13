use std::str::FromStr;

type Vec2D = (i32, i32);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up = 1,
    Left = 2,
    Down = 3,
    Right = 4,
}

impl Direction {
    pub fn to_vec(&self) -> Vec2D {
        match self {
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
        }
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "L" => Ok(Self::Left),
            "D" => Ok(Self::Down),
            "R" => Ok(Self::Right),
            _ => Err("Unknown str, cannot parse.".to_string()),
        }
    }
}

fn parse_input_line(line: &str) -> Result<(Direction, u32), String> {
    let (direction, magnitude) = match line
        .split_once(' ')
        .ok_or_else(|| "Failed to split line in 2".to_string())
    {
        Ok(pair) => pair,
        Err(e) => return Err(e),
    };

    let direction: Direction = match direction.parse() {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    let magnitude: u32 = match magnitude.parse() {
        Ok(d) => d,
        Err(e) => return Err(e.to_string()),
    };
    Ok((direction, magnitude))
}

pub fn parse_input(input: &str) -> Result<Vec<(Direction, u32)>, String> {
    input.lines().map(parse_input_line).collect()
}

pub fn generate_history(data: &[(Direction, u32)]) -> Vec<(Vec2D, Vec2D)> {
    data.iter()
        .flat_map(|(direction, magnitude)| vec![direction.to_vec(); *magnitude as usize])
        .fold(vec![], |mut history, (dx, dy)| {
            history.push(match history.last() {
                Some(((hx, hy), tail)) => {
                    let head = (hx + dx, hy + dy);
                    let tail = calculate_tail_position(head, *tail);
                    dbg!((head, tail))
                }
                None => ((dx, dy), (dx, dy)),
            });
            history
        })
}

pub fn calculate_tail_position((hx, hy): Vec2D, (tx, ty): Vec2D) -> Vec2D {
    let dx = hx - tx;
    let dy = hx - hy;
    let mut t2x = 0;
    if dx.abs() == 2 {
        t2x = tx + dx - 1;
    } else {
        t2x = tx;
    }
    let t2y = ty + if dy == 2 {
        1
    } else {
        0
    };
    (t2x, t2y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    mod part1 {
        use super::*;
        #[test]
        fn example_works() {
            let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

            let data = parse_input(input).unwrap();
            let history = generate_history(&data);
            println!("history: {:?}", history);
            let (minx, maxx, miny, maxy) = history.iter().fold(
                (i32::max_value(), i32::min_value(), i32::max_value(), i32::min_value()),
                |(minx, maxx, miny, maxy), ((hx, hy), (tx, ty))| {
                    (
                        [minx, *hx, *tx].iter().min().copied().unwrap(),
                        [maxx, *hx, *tx].iter().max().copied().unwrap(),
                        [miny, *hy, *ty].iter().min().copied().unwrap(),
                        [maxy, *hy, *ty].iter().max().copied().unwrap(),
                    )
                },
            );
            println!("{minx}, {maxx}; {miny}, {maxy}");

            history
                .iter()
                .for_each(|((hx, hy), (tx, ty))| {
                    println!(
                        "{}\n",
                        (minx + 1..maxx)
                            .rev()
                            .map(|i| {
                                (miny..maxy + 1)
                                    .map(|j| {
                                        let mut c = '.';
                                        if i == *tx && j == *ty {
                                            c = 'T'
                                        }

                                        if i == *hx && j == *hy {
                                            c = 'H'
                                        }
                                        c
                                    })
                                    .collect::<String>()
                            })
                            .collect::<Vec<_>>()
                            .join("\n")
                    );
                });

            let result: Vec<Vec2D> = history.iter().map(|(_, t)| t).fold(vec![], |mut acc, t| {
                println!("checking {:?}", t);
                if !acc.contains(t) {
                    println!(" -> adding");
                    acc.push(*t)
                }
                acc
            });
            println!("{:?}", result);
            assert_eq!(result.len(), 13);
        }
    }

    /// .....    .....    .....
    /// .TH.. -> .T.H. -> ..TH.
    /// .....    .....    .....
    ///
    /// ...    ...    ...
    /// .T.    .T.    ...
    /// .H. -> ... -> .T.
    /// ...    .H.    .H.
    /// ...    ...    ...
    #[test_case((1, 3), (1, 1), (1, 2))]
    #[test_case((3, 1), (1, 1), (2, 1))]
    ///
    /// .....    .....    .....
    /// .....    ..H..    ..H..
    /// ..H.. -> ..... -> ..T..
    /// .T...    .T...    .....
    /// .....    .....    .....
    ///
    /// .....    .....    .....
    /// .....    .....    .....
    /// ..H.. -> ...H. -> ..TH.
    /// .T...    .T...    .....
    /// .....    .....    .....
    #[test_case((1, 2), (3, 1), (2, 2))]
    #[test_case((2, 3), (3, 1), (2, 2))]
    fn tail_position_calculator(head: Vec2D, old_tail: Vec2D, new_tail: Vec2D) {
        assert_eq!(calculate_tail_position(head, old_tail), new_tail);
    }

    #[test_case(1, "R 4", Direction::Right, 4)]
    #[test_case(2, "U 4", Direction::Up, 4)]
    #[test_case(3, "L 3", Direction::Left, 3)]
    #[test_case(4, "D 1", Direction::Down, 1)]
    #[test_case(5, "R 4", Direction::Right, 4)]
    #[test_case(6, "D 1", Direction::Down, 1)]
    #[test_case(7, "L 5", Direction::Left, 5)]
    #[test_case(8, "R 2", Direction::Right, 2)]
    fn test_input_parser_success(
        _: usize,
        input: &str,
        expected_direction: Direction,
        expected_magnitude: u32,
    ) {
        let (actual_direction, actual_magnitude) = parse_input_line(input).unwrap();

        assert_eq!(actual_direction, expected_direction);
        assert_eq!(actual_magnitude, expected_magnitude);
    }
    #[test_case("f 4")]
    #[test_case(". 4")]
    #[test_case("asd1")]
    #[test_case("asadszxc")]
    #[test_case("R -5")]
    #[test_case("D abc")]
    #[test_case("")]
    fn test_input_parser_failures(input: &str) {
        assert!(parse_input_line(input).is_err());
    }
}
