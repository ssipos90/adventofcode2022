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
        let tail = pair[1];

        let head = match acc.last() {
            Some(head) => *head,
            None => {
                let (hx, hy) = pair[0];
                let head = (hx + dx, hy + dy);
                acc.push(head);
                head
            }
        };

        acc.push(calculate_tail_position(head, tail));

        acc
    })
}

pub fn generate_motions(data: &[Vec2D], length: usize) -> Vec<Vec<Vec2D>> {
    data.iter()
        .fold(vec![vec![data[0]; length]], |mut acc, motion| {
            let prev = acc.last().unwrap();
            acc.push(apply_motion(prev, motion));

            acc
        })
}

pub fn calculate_tail_position((h2x, h2y): Vec2D, (tx, ty): Vec2D) -> Vec2D {
    let dx = h2x.abs_diff(tx);
    let dy = h2y.abs_diff(ty);

    if dx > 1 || dy > 1 {
        (
            tx + (h2x - tx).cmp(&0) as i32,
            ty + (h2y - ty).cmp(&0) as i32,
        )
    } else {
        (tx, ty)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use test_case::test_case;

    fn test_helper(input: &str, length: usize) -> (Vec<Vec<Vec2D>>, usize) {
        let data = parse_input(input).unwrap();
        let history = generate_motions(&data, length);

        let mut visited: HashSet<Vec2D> = HashSet::new();
        println!("{:?}", history);

        history.iter().for_each(|v| {
            visited.insert(*v.last().unwrap());
        });

        (history, visited.len())
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

            let (_history, results) = test_helper(input, 2);

            // print_history(&_history);

            assert_eq!(results, 13);
        }

        #[test]
        fn input_works() {
            let input = std::fs::read_to_string("input").unwrap();
            let (_history, results) = test_helper(&input, 2);
            assert_eq!(results, 6190);
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

            // print_history(&_history);

            assert_eq!(results, 36);
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
    #[test_case("left to right", (1, 3), (1, 1), (1, 2))]
    #[test_case("right to left", (1, 1), (1, 3), (1, 2))]
    #[test_case("top to bottom", (3, 1), (1, 1), (2, 1))]
    #[test_case("bottom to top", (3, 1), (1, 1), (2, 1))]
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
    #[test_case("diagonal", (1, 2), (3, 1), (2, 2))]
    #[test_case("diagonal", (2, 3), (3, 1), (2, 2))]
    fn tail_position_calculator(_: &str, new_head: Vec2D, old_tail: Vec2D, new_tail: Vec2D) {
        assert_eq!(calculate_tail_position(new_head, old_tail), new_tail);
    }

    #[test]
    fn scenario_works() {
        let moves = [
            (0, ((4, 1), (4, 0), (4, 0))),
            (1, ((4, 2), (4, 0), (4, 1))),
            (2, ((4, 3), (4, 1), (4, 2))),
            (3, ((4, 4), (4, 2), (4, 3))),
            (4, ((3, 4), (4, 3), (4, 3))),
            (5, ((2, 4), (4, 3), (3, 4))),
            (6, ((1, 4), (3, 4), (2, 4))),
            (7, ((0, 4), (2, 4), (1, 4))),
            (8, ((0, 3), (1, 4), (1, 4))),
            (9, ((0, 2), (1, 4), (0, 3))),
            (10, ((0, 1), (0, 3), (0, 2))),
            (11, ((1, 1), (0, 2), (0, 2))),
            (12, ((1, 2), (0, 2), (0, 2))),
            (13, ((1, 3), (0, 2), (0, 2))),
            (14, ((1, 4), (0, 2), (1, 3))),
            (15, ((1, 5), (1, 3), (1, 4))),
            (16, ((2, 5), (1, 4), (1, 4))),
            (17, ((2, 4), (1, 4), (1, 4))),
            (18, ((2, 3), (1, 4), (1, 4))),
            (19, ((2, 2), (1, 4), (2, 3))),
            (20, ((2, 1), (2, 3), (2, 2))),
            (21, ((2, 0), (2, 2), (2, 1))),
            (22, ((2, 1), (2, 1), (2, 1))),
            (23, ((2, 2), (2, 1), (2, 1))),
        ];
        for (_idx, (new_head, old_tail, new_tail)) in moves {
            assert_eq!(calculate_tail_position(new_head, old_tail), new_tail);
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

    #[test_case( &[(0, 5), (0, 4), (0, 3)], &(0, 1), &[(0, 6), (0, 5), (0, 4)] )]
    #[test_case( &[(1, 0), (0, 1), (0, 2)], &(1, 0), &[(2, 0), (1, 0), (1, 1)] )]
    fn motion_gets_applied(initial: &[Vec2D], d: &Vec2D, expect: &[Vec2D]) {
        assert_eq!(apply_motion(initial, d), expect);
    }

    // fn print_history(history: &[Vec<Vec2D>]) {
    //     let (minx, maxx, miny, maxy) = history.iter().flatten().fold(
    //         (
    //             i32::max_value(),
    //             i32::min_value(),
    //             i32::max_value(),
    //             i32::min_value(),
    //         ),
    //         |(minx, maxx, miny, maxy), (x, y)| {
    //             (minx.min(*x), maxx.max(*x), miny.min(*y), maxy.max(*y))
    //         },
    //     );
    //     println!("{minx}, {maxx}; {miny}, {maxy}");

    //     history.iter().for_each(|frame| {
    //         println!(
    //             "{}\n",
    //             (minx..maxx)
    //                 .rev()
    //                 .map(|i| {
    //                     (miny..maxy)
    //                         .map(|j| {
    //                             match frame
    //                                 .iter()
    //                                 .enumerate()
    //                                 .find(|&(_, &(x, y))| i == x && j == y)
    //                             {
    //                                 Some((0, _)) => 'H',
    //                                 Some((idx, _)) => idx.to_string().chars().next().unwrap(),
    //                                 None => '.',
    //                             }
    //                         })
    //                         .collect::<String>()
    //                 })
    //                 .collect::<Vec<_>>()
    //                 .join("\n")
    //         );
    //     });
    // }
}
