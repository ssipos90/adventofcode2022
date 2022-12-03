pub fn calculate_priority(c: char) -> Result<u32, String> {
    let c1 = c as i32;
    if (65..=91).contains(&c1) {
        return Ok((38 - c1).unsigned_abs());
    }
    if (97..=123).contains(&c1) {
        return Ok((96 - c1).unsigned_abs());
    }
    Err("Out of range".into())
}

pub fn calculate_total_priority(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let l = line.len();
        let l1 = &line[..l / 2];
        let l2 = &line[l / 2..];

        acc + l1
            .chars()
            .find_map(|c1| match l2.chars().any(|c2| c1 == c2) {
                true => Some(calculate_priority(c1).unwrap()),
                false => None,
            })
            .unwrap()
    })
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;
    use test_case::test_case;

    #[test]
    fn example_works() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

        assert_eq!(calculate_total_priority(input), 157);
    }

    #[test]
    fn input_works() {
        let input = read_to_string("src/day3").unwrap();
        assert_eq!(calculate_total_priority(&input), 7997);
    }

    #[test_case('a', 1)]
    #[test_case('m', 13)]
    #[test_case('y', 25)]
    #[test_case('A', 27)]
    #[test_case('Q', 43)]
    #[test_case('Z', 52)]
    fn priorities(c: char, score: u32) {
        let p = calculate_priority(c).unwrap();
        assert_eq!(p, score);
    }
}
