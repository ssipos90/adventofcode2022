pub fn parse_input(input: &str) -> Result<Vec<Vec<i32>>, String> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or_else(|| "Failed to parse char as digit.".to_string())
                        .map(|c| c as i32)
                })
                .collect::<Result<Vec<i32>, String>>()
        })
        .collect()
}

pub fn count_visible(forest: Vec<Vec<i32>>) -> usize {
    let m = forest.len();
    let n = forest[0].len();
    if m < 3 {
        return m * n;
    }
    if n < 3 {
        return m * n;
    }
    // for row in forest.iter() {
    //     println!("{:?}", row);
    // }
    // println!();
    forest.iter().enumerate().fold(0, |acc, (i, row)| {
        // eprintln!("{:?}", row);
        acc + row
            .iter()
            .enumerate()
            .filter(|(j, &c)| {
                let j = *j;
                // eprintln!("{i}-{j}: {c}");

                if i == 0 || i == m - 1 {
                    // eprintln!("    visible, top or bottom edge");
                    return true;
                }

                if j == 0 || j == n - 1 {
                    // eprintln!("    visible, left or right edge");
                    return true;
                }
                if row[0..j].iter().all(|&x| x < c) {
                    // eprintln!("   visible from left");
                    return true;
                }
                if row[j + 1..n].iter().all(|&x| x < c) {
                    // eprintln!("   visible from right");
                    return true;
                }

                if forest[0..i].iter().all(|row| row[j] < c) {
                    // eprintln!("   visible from top");
                    return true;
                }
                if forest[i + 1..m].iter().all(|row| row[j] < c) {
                    // eprintln!("   visible from bottom");
                    return true;
                }

                // eprintln!("   hidden");
                false
            })
            .count()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![vec![1,2,3]], 3)]
    #[test_case(vec![vec![1,2,3], vec![1,2,3]], 6)]
    #[test_case(vec![vec![1], vec![2]], 2)]
    #[test_case(vec![vec![1], vec![2], vec![3]], 3)]
    fn test_count_visible_on_small_forest(forest: Vec<Vec<i32>>, visible: usize) {
        assert_eq!(count_visible(forest), visible);
    }

    mod part1 {
        use std::fs::read_to_string;

        use super::*;
        #[test]
        fn example_works() {
            let input = r#"30373
25512
65332
33549
35390"#;

            let forest = parse_input(input).unwrap();
            assert_eq!(count_visible(forest), 21);
        }

        #[test]
        fn input_works() {
            let input = read_to_string("input").unwrap();

            let forest = parse_input(&input).unwrap();
            assert_eq!(count_visible(forest), 1825);
        }
    }
}
