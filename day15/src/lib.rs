use std::collections::BTreeMap;

type Coord = (isize, isize);

pub fn parse_input(input: &str) -> Result<Vec<(Coord, Coord)>, &'static str> {
    input.lines().map(parse_line).collect()
}

pub fn parse_line(input: &str) -> Result<(Coord, Coord), &'static str> {
    let parts = input.split(' ').collect::<Vec<&str>>();

    if parts.len() != 10 {
        return Err("Line should have 10 words.");
    }

    let x1 = parse_value(parts[2].trim_end_matches(','))?;
    let y1 = parse_value(parts[3].trim_end_matches(':'))?;
    let x2 = parse_value(parts[8].trim_end_matches(','))?;
    let y2 = parse_value(parts[9])?;

    Ok(((x1, y1), (x2, y2)))
}

fn parse_value(input: &str) -> Result<isize, &'static str> {
    let (_, value) = input
        .split_once('=')
        .ok_or("Could not split value on equals.")?;

    value
        .parse::<isize>()
        .map_err(|_| "Failed to parse value as isize.")
}

pub fn work_and_sweat(input: &[(Coord, Coord)], line_number: isize) -> usize {
    let distances: BTreeMap<&Coord, isize> = input
        .iter()
        .map(|(sensor, beacon)| {
            (
                sensor,
                (beacon.0.abs_diff(sensor.0) + beacon.1.abs_diff(sensor.0)) as isize,
            )
        })
        .collect();

    let _smf = distances.iter().filter(|(sensor, distance)| {
        ((sensor.1 - **distance)..(sensor.1 + **distance)).contains(&line_number)
    })
    .map(|(sensor, max_distance)| {
        // let width = distance * 2 + 1;
        let distance_to_line = sensor.1 - line_number;
        let direction_to_line = distance_to_line.signum();
        // let max_distance_on_line = (max_d);
        todo!()
    })
    .collect::<Vec<_>>();
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Sensor at x=2, y=18: closest beacon is at x=-2, y=15", ((2, 18), (-2, 15)))]
    #[test_case("Sensor at x=9, y=16: closest beacon is at x=10, y=16", ((9, 16), (10, 16)))]
    #[test_case("Sensor at x=13, y=2: closest beacon is at x=15, y=3", ((13, 2), (15, 3)))]
    #[test_case("Sensor at x=12, y=14: closest beacon is at x=10, y=16", ((12, 14), (10, 16)))]
    #[test_case("Sensor at x=10, y=20: closest beacon is at x=10, y=16", ((10, 20), (10, 16)))]
    #[test_case("Sensor at x=14, y=17: closest beacon is at x=10, y=16", ((14, 17), (10, 16)))]
    #[test_case("Sensor at x=8, y=7: closest beacon is at x=2, y=10", ((8, 7), (2, 10)))]
    #[test_case("Sensor at x=2, y=0: closest beacon is at x=2, y=10", ((2, 0), (2, 10)))]
    #[test_case("Sensor at x=0, y=11: closest beacon is at x=2, y=10", ((0, 11), (2, 10)))]
    #[test_case("Sensor at x=20, y=14: closest beacon is at x=25, y=17", ((20, 14), (25, 17)))]
    #[test_case("Sensor at x=17, y=20: closest beacon is at x=21, y=22", ((17, 20), (21, 22)))]
    #[test_case("Sensor at x=16, y=7: closest beacon is at x=15, y=3", ((16, 7), (15, 3)))]
    #[test_case("Sensor at x=14, y=3: closest beacon is at x=15, y=3", ((14, 3), (15, 3)))]
    #[test_case("Sensor at x=20, y=1: closest beacon is at x=15, y=3", ((20, 1), (15, 3)))]
    fn line_parser(input: &str, expected_output: (Coord, Coord)) {
        let output = parse_line(input).unwrap();
        assert_eq!(output, expected_output);
    }

    #[test_case("", "Line should have 10 words.")]
    #[test_case("a, b, c", "Line should have 10 words.")]
    #[test_case(
        "Sensor at x=20.3, y=1: closest beacon is at x=15, y=3",
        "Failed to parse value as isize."
    )]
    #[test_case(
        "Sensor at x=20, y=abc: closest beacon is at x=15, y=3",
        "Failed to parse value as isize."
    )]
    #[test_case(
        "Sensor at x=20, y=1: closest beacon is at x=, y=3",
        "Failed to parse value as isize."
    )]
    #[test_case(
        "Sensor at x=20, y=2: closest beacon is at x=15, y=-abc",
        "Failed to parse value as isize."
    )]
    fn line_parser_failures(input: &str, expected_err: &str) {
        let out = parse_line(input);

        assert_eq!(out.unwrap_err(), expected_err);
    }

    mod part1 {
        use super::*;

        #[test]
        fn example_works() {
            let input = parse_input(include_str!("../example")).unwrap();

            assert_eq!(work_and_sweat(&input, 2000000), 20);
        }
    }
}
