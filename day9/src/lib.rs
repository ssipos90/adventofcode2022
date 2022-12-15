type Vec2D = (i32, i32);

fn parse_input_line(line: &str) -> Result<Vec2D, String> {
    let (direction, magnitude) = match line
        .split_once(' ')
        .ok_or_else(|| "Failed to split line in 2".to_string())
    {
        Ok(pair) => pair,
        Err(e) => return Err(e),
    };

    let magnitude: i32 = match magnitude.parse::<u32>() {
        Ok(d) => d as i32,
        Err(e) => return Err(e.to_string()),
    };

    match direction {
        "U" => Ok((0, -magnitude)),
        "L" => Ok((-magnitude, 0)),
        "D" => Ok((0, magnitude)),
        "R" => Ok((magnitude, 0)),
        _ => Err("Unknown str, cannot parse.".to_string()),
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Vec2D>, String> {
    input.lines().map(parse_input_line).collect()
}

fn apply_motion(prev: &[Vec2D], (dx, dy): &Vec2D) -> Vec<Vec2D> {
    prev.windows(2).fold(vec![], |mut acc, pair| {
        let old_head @ (hx, hy) = pair[0];
        let tail = pair[1];

        let head = match acc.last() {
            Some(head) => *head,
            None => {
                let head = (hx + dx, hy + dy);
                acc.push(head);
                head
            }
        };

        acc.push(calculate_tail_position(old_head, head, tail));

        acc
    })
}

pub fn generate_motions(data: &[Vec2D], length: usize) -> Vec<Vec<Vec2D>> {
    data.iter().map(|(dx, dy)| {
        let motion = (dx.cmp(&0) as i32, dy.cmp(&0) as i32);
        let initial = vec![motion; length];
        let mut acc = vec![apply_motion(&initial, &motion)];

        (0..dx.abs() + dy.abs()).for_each(|_| {
            if let Some(prev) = acc.last() {
                acc.push(apply_motion(prev, &motion));
            }
        });

        acc.last().unwrap().clone()
    })
    .collect()
}

pub fn calculate_tail_position((hx, hy): Vec2D, (h2x, h2y): Vec2D, (tx, ty): Vec2D) -> Vec2D {
    let dx = h2x.abs_diff(tx);
    let dy = h2y.abs_diff(ty);

    if dx > 1 || dy > 1 {
        (hx, hy)
    } else {
        (tx, ty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn test_helper(input: &str, length: usize) -> (Vec<Vec<Vec2D>>, Vec<Vec2D>) {
        let data = parse_input(input).unwrap();
        let history = generate_motions(&data, length);

        let result: Vec<Vec2D> =
            history
                .iter()
                .map(|v| v.last().unwrap())
                .fold(vec![], |mut acc, t| {
                    // println!("checking {:?}", t);
                    if !acc.contains(t) {
                        // println!(" -> adding");
                        acc.push(*t)
                    }
                    acc
                });

        (history, result)
    }
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

            let (history, results) = test_helper(input, 2);

            print_history(&history);

            assert_eq!(1 + results.len(), 13);
        }

        #[test]
        fn input_works() {
            let input = std::fs::read_to_string("input").unwrap();
            let (_history, results) = test_helper(&input, 2);
            assert_eq!(1 + results.len(), 6190);
        }

        mod example_history {
            fn parse_input((refx, refy): (i32, i32), input: &str) -> Vec<((i32, i32), (i32, i32))> {
                input
                    .split("\n\n")
                    .map(|chunk| {
                        let head = chunk
                            .lines()
                            .enumerate()
                            .find_map(|(i, line)| {
                                line.chars().enumerate().find_map(|(j, c)| match c {
                                    'H' => Some((refx + (i as i32), refy + (j as i32))),
                                    _ => None,
                                })
                            })
                            .unwrap();
                        let tail = chunk
                            .lines()
                            .enumerate()
                            .find_map(|(i, line)| {
                                line.chars().enumerate().find_map(|(j, c)| match c {
                                    'T' => Some((refx + (i as i32), refy + (j as i32))),
                                    _ => None,
                                })
                            })
                            .unwrap_or(head);
                        (head, tail)
                    })
                    .collect()
            }

            #[test]
            #[ignore]
            fn example_history() {
                let input = std::fs::read_to_string("example_history").unwrap();
                parse_input((0, 0), &input)
                    .windows(2)
                    .enumerate()
                    .for_each(|(i, window)| {
                        let (old_head, old_tail) = window[0];
                        let (head, tail) = window[1];
                        println!(
                            r#"({}, ({:?}, {:?}, {:?}, {:?})),"#,
                            i, old_head, head, old_tail, tail
                        );
                    });
            }
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example_works() {
            let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
            let (_history, results) = test_helper(input, 10);

            print_history(&_history);

            assert_eq!(1 + results.len(), 36);
        }
    }

    /// .....    .....    .....
    /// .TH.. -> .T.H. -> ..TH.
    /// .....    .....    .....
    ///
    /// .....    .....    .....
    /// ..HT. -> .H.T. -> .HT..
    /// .....    .....    .....
    ///
    /// ...    ...    ...
    /// .T.    .T.    ...
    /// .H. -> ... -> .T.
    /// ...    .H.    .H.
    /// ...    ...    ...
    #[test_case("left to right", (1, 2), (1, 3), (1, 1), (1, 2))]
    #[test_case("right to left", (1, 2), (1, 1), (1, 3), (1, 2))]
    #[test_case("top to bottom", (2, 1), (3, 1), (1, 1), (2, 1))]
    #[test_case("bottom to top", (2, 1), (3, 1), (1, 1), (2, 1))]
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
    #[test_case("diagonal", (2, 2), (1, 2), (3, 1), (2, 2))]
    #[test_case("diagonal", (2, 2), (2, 3), (3, 1), (2, 2))]
    ///
    /// Provided scenario
    fn tail_position_calculator(
        _: &str,
        old_head: Vec2D,
        new_head: Vec2D,
        old_tail: Vec2D,
        new_tail: Vec2D,
    ) {
        assert_eq!(
            calculate_tail_position(old_head, new_head, old_tail),
            new_tail
        );
    }

    #[test]
    fn scenario_works() {
        let moves = [
            (0, ((4, 0), (4, 1), (4, 0), (4, 0))),
            (1, ((4, 1), (4, 2), (4, 0), (4, 1))),
            (2, ((4, 2), (4, 3), (4, 1), (4, 2))),
            (3, ((4, 3), (4, 4), (4, 2), (4, 3))),
            (4, ((4, 4), (3, 4), (4, 3), (4, 3))),
            (5, ((3, 4), (2, 4), (4, 3), (3, 4))),
            (6, ((2, 4), (1, 4), (3, 4), (2, 4))),
            (7, ((1, 4), (0, 4), (2, 4), (1, 4))),
            (8, ((0, 4), (0, 3), (1, 4), (1, 4))),
            (9, ((0, 3), (0, 2), (1, 4), (0, 3))),
            (10, ((0, 2), (0, 1), (0, 3), (0, 2))),
            (11, ((0, 1), (1, 1), (0, 2), (0, 2))),
            (12, ((1, 1), (1, 2), (0, 2), (0, 2))),
            (13, ((1, 2), (1, 3), (0, 2), (0, 2))),
            (14, ((1, 3), (1, 4), (0, 2), (1, 3))),
            (15, ((1, 4), (1, 5), (1, 3), (1, 4))),
            (16, ((1, 5), (2, 5), (1, 4), (1, 4))),
            (17, ((2, 5), (2, 4), (1, 4), (1, 4))),
            (18, ((2, 4), (2, 3), (1, 4), (1, 4))),
            (19, ((2, 3), (2, 2), (1, 4), (2, 3))),
            (20, ((2, 2), (2, 1), (2, 3), (2, 2))),
            (21, ((2, 1), (2, 0), (2, 2), (2, 1))),
            (22, ((2, 0), (2, 1), (2, 1), (2, 1))),
            (23, ((2, 1), (2, 2), (2, 1), (2, 1))),
        ];
        for (_idx, (old_head, new_head, old_tail, new_tail)) in moves {
            assert_eq!(
                calculate_tail_position(old_head, new_head, old_tail),
                new_tail
            );
        }
    }

    #[test_case(1, "R 4", (4, 0))]
    #[test_case(2, "U 4", (0, -4))]
    #[test_case(3, "L 3", (-3, 0))]
    #[test_case(4, "D 1", (0, 1))]
    #[test_case(5, "D 4", (0, 4))]
    #[test_case(6, "L 5", (-5, 0))]
    #[test_case(7, "R 2", (2, 0))]
    fn test_input_parser_success(_: usize, input: &str, v: Vec2D) {
        assert_eq!(parse_input_line(input).unwrap(), v);
    }
    #[test_case("f 4")]
    #[test_case(". 4")]
    #[test_case("asd1")]
    #[test_case("asadszxc")]
    #[test_case("R -5")]
    #[test_case("D abc")]
    #[test_case("")]
    fn test_input_parser_failures(line: &str) {
        assert!(parse_input_line(line).is_err());
    }

    fn print_history(history: &[Vec<Vec2D>]) {
        let (minx, maxx, miny, maxy) = history.iter().flatten().fold(
            (
                i32::max_value(),
                i32::min_value(),
                i32::max_value(),
                i32::min_value(),
            ),
            |(minx, maxx, miny, maxy), (x, y)| {
                (minx.min(*x), maxx.max(*x), miny.min(*y), maxy.max(*y))
            },
        );
        println!("{minx}, {maxx}; {miny}, {maxy}");

        history.iter().for_each(|frame| {
            println!(
                "{}\n",
                (minx..maxx)
                    .rev()
                    .map(|i| {
                        (miny..maxy)
                            .map(|j| {
                                match frame
                                    .iter()
                                    .enumerate()
                                    .find(|&(_, &(x, y))| i == x && j == y)
                                {
                                    Some((0, _)) => 'H',
                                    Some((idx, _)) => idx.to_string().chars().next().unwrap(),
                                    None => '.',
                                }
                            })
                            .collect::<String>()
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        });
    }
}
