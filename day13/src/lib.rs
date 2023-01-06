pub fn process_signal(input: &str) -> Result<Vec<bool>, String> {
    input
        .split("\n\n")
        .map(|p| {
            p.split_once('\n')
                .ok_or_else(|| "Failed to split pair.".to_string())
                .and_then(|(a, b)| {
                    println!("left: {a}, right: {b}");
                    process_packet_list(a)?;
                    process_packet_list(b)?;
                    Ok(true)
                })
        })
        .collect()
}

enum Sequence {
    List(Vec<Sequence>),
    Number(u32),
}

impl ToString for Sequence {
    fn to_string(&self) -> String {
        match self {
            Sequence::List(list) => {
                let mut s: String = String::new();
                s.push('[');
                s.push_str(
                    &list
                        .iter()
                        .map(|item| item.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
                s.push(']');
                s
            }
            Sequence::Number(number) => number.to_string(),
        }
    }
}

fn process_packet(input: &str) -> Result<Sequence, String> {
    let c = input
        .chars()
        .next()
        .ok_or_else(|| "Failed to get first char of str.".to_string())?;
    Ok(match c {
        '[' => Sequence::List(process_packet_list(&input[1..input.len() - 1])?),
        _ => Sequence::Number(process_packet_number(input)?),
    })
}

fn process_packet_list(input: &str) -> Result<Vec<Sequence>, String> {
    input.split(',').map(process_packet).collect()
}

fn process_packet_number(input: &str) -> Result<u32, String> {
    input.parse::<u32>().map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case('a', "[1,1,3,1,1]")]
    #[test_case('b', "[1,1,5,1,1]")]
    #[test_case('c', "[[1],[2,3,4]]")]
    #[test_case('d', "[[1],4]")]
    #[test_case('e', "[9]")]
    #[test_case('f', "[[8,7,6]]")]
    #[test_case('g', "[[4,4],4,4]")]
    #[test_case('h', "[[4,4],4,4,4]")]
    #[test_case('i', "[7,7,7,7]")]
    #[test_case('j', "[7,7,7]")]
    #[test_case('k', "[]")]
    #[test_case('l', "[3]")]
    #[test_case('m', "[[[]]]")]
    #[test_case('n', "[[]]")]
    #[test_case('o', "[1,[2,[3,[4,[5,6,7]]]],8,9]")]
    #[test_case('p', "[1,[2,[3,[4,[5,6,0]]]],8,9]")]
    fn packet_processor(_: char, input: &str) {
        println!("{input}");
        assert_eq!(process_packet(input).unwrap().to_string(), input);
    }

    mod part1 {
        use super::*;

        #[test]
        fn example_works() {
            let input = include_str!("../example");

            let result: usize = process_signal(input)
                .unwrap()
                .iter()
                .enumerate()
                .filter_map(|(i, correct)| correct.then(|| i + 1))
                .sum();
            assert_eq!(result, 13);
        }
    }
}
