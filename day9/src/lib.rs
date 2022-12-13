use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up = 1,
    Left = 2,
    Down = 3,
    Right = 4,
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

pub fn smf(data: &[(Direction, u32)]) -> u32 {
    let _directions: Vec<Direction> = data
        .iter()
        .flat_map(|(direction, magnitude)| vec![*direction; *magnitude as usize])
        .collect();

    1
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
            let result = smf(&data);
            assert_eq!(result, 13);
        }
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
