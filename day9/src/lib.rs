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

pub fn generate_history(data: &[Vec2D]) -> Vec<(Vec2D, Vec2D)> {
    data.iter()
        .flat_map(|(x, y)| {
            let v = (x.cmp(&0) as i32, y.cmp(&0) as i32);

            (0..x.abs() + y.abs()).map(move |_| v)
        })
        .fold(vec![], |mut history, (dx, dy)| {
            history.push(match history.last() {
                Some((old_head @ (hx, hy), tail)) => {
                    let head = (hx + dx, hy + dy);
                    let tail = calculate_tail_position(*old_head, head, *tail);
                    (head, tail)
                }
                None => ((dx, dy), (dx, dy)),
            });
            history
        })
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

    mod part1 {
        use super::*;
        fn test_helper(input: &str) -> (Vec<(Vec2D, Vec2D)>, Vec<Vec2D>) {
            let data = parse_input(input).unwrap();
            let history = generate_history(&data);
            // println!("history: {:?}", history);
            // let (minx, maxx, miny, maxy) = history.iter().fold(
            //     (
            //         i32::max_value(),
            //         i32::min_value(),
            //         i32::max_value(),
            //         i32::min_value(),
            //     ),
            //     |(minx, maxx, miny, maxy), ((hx, hy), (tx, ty))| {
            //         (
            //             [minx, *hx, *tx].iter().min().copied().unwrap(),
            //             [maxx, *hx, *tx].iter().max().copied().unwrap(),
            //             [miny, *hy, *ty].iter().min().copied().unwrap(),
            //             [maxy, *hy, *ty].iter().max().copied().unwrap(),
            //         )
            //     },
            // );
            // println!("{minx}, {maxx}; {miny}, {maxy}");

            // history.iter().for_each(|((hx, hy), (tx, ty))| {
            //     println!(
            //         "{}\n",
            //         (minx + 1..maxx)
            //             .rev()
            //             .map(|i| {
            //                 (miny..maxy + 1)
            //                     .map(|j| {
            //                         let mut c = '.';
            //                         if i == *tx && j == *ty {
            //                             c = 'T'
            //                         }

            //                         if i == *hx && j == *hy {
            //                             c = 'H'
            //                         }
            //                         c
            //                     })
            //                     .collect::<String>()
            //             })
            //             .collect::<Vec<_>>()
            //             .join("\n")
            //     );
            // });

            let result: Vec<Vec2D> = history.iter().map(|(_, t)| t).fold(vec![], |mut acc, t| {
                // println!("checking {:?}", t);
                if !acc.contains(t) {
                    // println!(" -> adding");
                    acc.push(*t)
                }
                acc
            });

            (history, result)
        }
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

            let (_history, results) = test_helper(input);

            assert_eq!(1 + results.len(), 13);
        }

        #[test]
        fn input_works() {
            let input = std::fs::read_to_string("input").unwrap();
            let (_history, results) = test_helper(&input);
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
}
