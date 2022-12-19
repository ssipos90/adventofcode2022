pub fn find_marker_of_len(len: usize, signal: &str) -> Option<usize> {
    signal
        .as_bytes()
        .windows(len)
        .enumerate()
        .find_map(|(i, b)| {
            if (1..b.len()).any(|c| b[c..].contains(&b[c - 1])) {
                None
            } else {
                Some(i + len)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    mod part1 {
        use super::*;
        use std::fs::read_to_string;
        use test_case::test_case;

        #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
        #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
        #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
        #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
        #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
        fn example_works(signal: &str, pos: usize) {
            assert_eq!(find_marker_of_len(4, signal).unwrap(), pos);
        }

        #[test]
        fn input_works() {
            let signal = read_to_string("input").unwrap();

            assert_eq!(find_marker_of_len(4, &signal).unwrap(), 1080);
        }
    }

    mod part2 {
        use super::*;
        use std::fs::read_to_string;
        use test_case::test_case;

        #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
        #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
        #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
        #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
        #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
        fn example_works(signal: &str, pos: usize) {
            assert_eq!(find_marker_of_len(14, signal).unwrap(), pos);
        }

        #[test]
        fn input_works() {
            let signal = read_to_string("input").unwrap();

            assert_eq!(find_marker_of_len(14, &signal).unwrap(), 3645);
        }
    }
}
