use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list0,
    sequence::delimited, IResult,
};

pub fn process_signal(input: &str) -> Result<Vec<bool>, String> {
    input
        .split("\n\n")
        .map(|p| {
            p.split_once('\n')
                .ok_or_else(|| "Failed to split pair.".to_string())
                .and_then(|(a, b)| {
                    let _left = process_packet(a)?;
                    let _right = process_packet(b)?;

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

fn process_list(input: &str) -> IResult<&str, Vec<Sequence>> {
    delimited(tag("["), separated_list0(tag(","), process_item), tag("]"))(input)
}

fn process_item(input: &str) -> IResult<&str, Sequence> {
    let number = map(nom::character::complete::u32, Sequence::Number);

    alt((number, map(process_list, Sequence::List)))(input)
}

fn process_packet(input: &str) -> Result<Sequence, String> {
    let (_, a) = process_item(input).map_err(|e| e.to_string())?;

    Ok(a)
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
        let packet = process_packet(input).unwrap();
        assert_eq!(packet.to_string().as_str(), input);
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
