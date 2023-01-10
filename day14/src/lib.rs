use std::io::{stdout, Write};

type Pair = (usize, usize);

pub fn parse_input(input: &str) -> Result<Vec<Vec<Pair>>, &'static str> {
    input
        .lines()
        .map(|line| {
            if line.is_empty() {
                Err("Line is empty.")
            } else {
                parse_line(line)
            }
        })
        .collect()
}

fn parse_line(line: &str) -> Result<Vec<Pair>, &'static str> {
    line.split_terminator(" -> ").map(parse_pair).collect()
}

fn parse_pair(pair: &str) -> Result<Pair, &'static str> {
    let (left, right) = pair.split_once(',').ok_or("Failed to split on comma.")?;
    let left = left
        .parse::<usize>()
        .map_err(|_| "Failed to parse left side.")?;
    let right = right
        .parse::<usize>()
        .map_err(|_| "Failed to parse right side.")?;

    Ok((left, right))
}

type World = Vec<Vec<usize>>;

pub fn build_world(source: Pair, obstacles: &[Vec<Pair>]) -> World {
    let (min_width, max_width, depth): (usize, usize, usize) =
        obstacles.iter().fold((usize::MAX, 0, 0), |acc, obstacle| {
            obstacle
                .iter()
                .fold(acc, |(min_width, max_width, max_depth), (y, x)| {
                    (min_width.min(*y), max_width.max(*y), max_depth.max(*x))
                })
        });
    let source = (source.0 - min_width + 1, source.1);
    let width = max_width - min_width + 1;

    let mut world = vec![vec![0; width + 1]; depth + 1];
    world[source.1][source.0] = 5;

    for obstacle in obstacles {
        for a in obstacle.windows(2) {
            if let [(x1, y1), (x2, y2)] = a {
                world[*x1][*y1 - min_width] = 1;
            } else {
                panic!("windows should work");
            }
        }
    }

    world
}

fn print_world(world: &[Vec<usize>]) {
    let mut s = stdout();
    for line in world {
        s.write_all(
            line.iter()
                .map(|&block| match block {
                    0 => '.',
                    1 => '#',
                    2 => 'o',
                    5 => '+',
                    _ => panic!("unknown block."),
                })
                .collect::<String>()
                .as_bytes(),
        )
        .unwrap();
        s.write_all([b'\n'].as_slice()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("498,4", (498,4))]
    #[test_case("498,6", (498,6))]
    #[test_case("496,6", (496,6))]
    #[test_case("503,4", (503,4))]
    #[test_case("502,4", (502,4))]
    #[test_case("502,9", (502,9))]
    #[test_case("494,9", (494,9))]
    fn pair_parser(input: &str, expected: Pair) {
        let pair = parse_pair(input).unwrap();
        assert_eq!(pair, expected);
    }

    #[test_case("", "Failed to split on comma.")]
    #[test_case(",", "Failed to parse left side.")]
    #[test_case("abc,sd", "Failed to parse left side.")]
    #[test_case("123,sd", "Failed to parse right side.")]
    fn pair_parser_failures(input: &str, expected_err: &str) {
        match parse_pair(input) {
            Ok(_) => panic!("Should have failed on {}.", input),
            Err(e) => assert_eq!(e, expected_err),
        };
    }

    #[test_case("", &[])]
    #[test_case("1,2", &[(1, 2)])]
    #[test_case("1,2 -> 3,4", &[(1, 2), (3, 4)])]
    fn line_parser(input: &str, expected: &[Pair]) {
        let line = parse_line(input).unwrap();
        assert_eq!(line.as_slice(), expected);
    }

    #[test_case(" -> ", "Failed to split on comma.")]
    #[test_case(",", "Failed to parse left side.")]
    #[test_case("123,", "Failed to parse right side.")]
    fn line_parser_failures(input: &str, expected_err: &str) {
        match parse_line(input) {
            Ok(_) => panic!(
                "Should have failed on '{}' with err: {}.",
                input, expected_err
            ),
            Err(e) => assert_eq!(e, expected_err),
        };
    }

    // test case for "" is not needed since "".lines() => [] not [""]
    // test case for "\n" is not needed since "abc\n".lines() => ["abc"] not ["abc", ""]
    #[test_case("\n", "Line is empty.")]
    #[test_case("\n\n", "Line is empty.")]
    #[test_case("1,2\n\n", "Line is empty.")]
    #[test_case("\n1,2\n", "Line is empty.")]
    #[test_case("1,2\n3,4\n\n5,6\n", "Line is empty.")]
    #[test_case("1,2\n\n3,4", "Line is empty.")]
    fn input_parser_failures(input: &str, expected_err: &str) {
        match parse_input(input) {
            Ok(_) => panic!(
                "Should have failed on '{}' with err: {}.",
                input, expected_err
            ),
            Err(e) => assert_eq!(e, expected_err),
        };
    }

    #[test_case("", &[])]
    #[test_case("1,2", &[vec![(1,2)]])]
    #[test_case("1,2 -> 3,4", &[vec![(1,2), (3,4)]])]
    #[test_case("1,2\n3,4", &[vec![(1,2)], vec![(3,4)]])]
    fn input_parser_successes(input: &str, expected: &[Vec<Pair>]) {
        let result = parse_input(input).unwrap();
        assert_eq!(result.as_slice(), expected);
    }

    mod part1 {
        use super::*;
        #[test]
        fn example_works() {
            let input = include_str!("../example");
            let obstacles = parse_input(input).unwrap();
            let world = build_world((500, 0), &obstacles);
            print_world(world.as_slice());
        }
    }
}
