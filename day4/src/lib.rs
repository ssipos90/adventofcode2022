type Interval = (u32, u32);

pub fn parse_line(line: &str) -> Result<(Interval, Interval), String> {
    let (l, r) = line
        .split_once(',')
        .ok_or_else(|| "Failed to split by ,".to_string())?;
    Ok((parse_pair(l)?, parse_pair(r)?))
}

fn parse_pair(interval: &str) -> Result<Interval, String> {
    let (l, r) = interval
        .split_once('-')
        .ok_or_else(|| "Failed to split by -".to_string())?;

    Ok((
        l.parse::<u32>()
            .map_err(|_| "First piece failed to convert to u32".to_string())?,
        r.parse::<u32>()
            .map_err(|_| "Second piece failed to convert to u32".to_string())?,
    ))
}

/// This can't be written clearly :)
pub fn are_pairs_inclusive(((a, b), (c, d)): &(Interval, Interval)) -> bool {
    (a <= c && b >= d) || (c <= a && d >= b)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    fn count_lines(input: &str) -> usize {
        input
            .lines()
            .map(|line| parse_line(line).unwrap())
            .filter(are_pairs_inclusive)
            .count()
    }

    #[test]
    fn example_works() {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

        assert_eq!(count_lines(input), 2);
    }

    #[test]
    fn input_works() {
        let input = read_to_string("input").unwrap();
        assert_eq!(count_lines(&input), 485);
    }

    mod parse_pair {
        use super::*;
        use test_case::test_case;

        #[test]
        fn fails_if_no_dash_found() {
            let a = parse_pair("smf");
            assert!(a.is_err(), "Not equal Err");
            assert_eq!(a.unwrap_err().as_str(), "Failed to split by -");
        }

        #[test_case("smf-1")]
        #[test_case("0.5-1")]
        #[test_case("-21312-1")]
        #[test_case("-1")]
        fn first_piece_fails_if_not_u32(pair: &str) {
            let a = parse_pair(pair);
            assert!(a.is_err());
            assert_eq!(
                a.unwrap_err().as_str(),
                "First piece failed to convert to u32"
            );
        }

        #[test_case("1-smf")]
        #[test_case("1-0.5")]
        #[test_case("1--21312")]
        #[test_case("1-")]
        fn second_piece_fails_if_not_u32(pair: &str) {
            let a = parse_pair(pair);
            assert!(a.is_err());
            assert_eq!(
                a.unwrap_err().as_str(),
                "Second piece failed to convert to u32"
            );
        }

        #[test_case("1-12312312", (1, 12312312))]
        #[test_case("1-1", (1, 1))]
        #[test_case("1312321312-1", (1312321312, 1))]
        #[test_case("1543645654-23476563", (1543645654, 23476563))]
        fn cool_pair(pair: &str, c: Interval) {
            let a = parse_pair(pair);
            assert!(a.is_ok());
            assert_eq!(a.unwrap(), c);
        }
    }
}
