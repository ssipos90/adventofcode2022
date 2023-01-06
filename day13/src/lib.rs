use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list0,
    sequence::delimited, IResult,
};

#[derive(Debug, PartialEq)]
enum PacketsOrder {
    Wrong = 1,
    Dunno,
    Right,
}

impl PacketsOrder {
    fn from_numbers(left: u32, right: u32) -> Self {
        match left.cmp(&right) {
            std::cmp::Ordering::Less => Self::Right,
            std::cmp::Ordering::Equal => Self::Dunno,
            std::cmp::Ordering::Greater => Self::Wrong,
        }
    }
}

fn check_order(left: Sequence, right: Sequence) -> PacketsOrder {
    match (left, right) {
        (Sequence::List(l), Sequence::List(r)) => check_list_order(l, r),
        (Sequence::List(l), r @ Sequence::Number(_)) => check_list_order(l, vec![r]),
        (l @ Sequence::Number(_), Sequence::List(r)) => check_list_order(vec![l], r),
        (Sequence::Number(l), Sequence::Number(r)) => PacketsOrder::from_numbers(l, r),
    }
}

fn check_list_order(left: Vec<Sequence>, right: Vec<Sequence>) -> PacketsOrder {
    let llen = left.len();
    let rlen = right.len();
    let result =
        left.into_iter()
            .zip(right)
            .find_map(|(left, right)| match check_order(left, right) {
                PacketsOrder::Dunno => None,
                o => Some(o),
            });

    match result {
        Some(o) => o,
        None => PacketsOrder::from_numbers(llen as u32, rlen as u32),
    }
}

pub fn parse_signal(input: &str) -> Result<Vec<(Sequence, Sequence)>, String> {
    input
        .split("\n\n")
        .map(|p| {
            p.split_once('\n')
                .ok_or_else(|| "Failed to split pair.".to_string())
                .and_then(|(a, b)| {
                    let left = parse_packet(a)?;
                    let right = parse_packet(b)?;
                    Ok((left, right))
                })
        })
        .collect()
}

pub fn check_signal(list: Vec<(Sequence, Sequence)>) -> Vec<bool> {
    list.into_iter()
        .map(|(left, right)| check_order(left, right) == PacketsOrder::Right)
        .collect()
}

pub enum Sequence {
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

fn parse_list(input: &str) -> IResult<&str, Vec<Sequence>> {
    delimited(tag("["), separated_list0(tag(","), parse_item), tag("]"))(input)
}

fn parse_item(input: &str) -> IResult<&str, Sequence> {
    let number = map(nom::character::complete::u32, Sequence::Number);

    alt((number, map(parse_list, Sequence::List)))(input)
}

fn parse_packet(input: &str) -> Result<Sequence, String> {
    let (_, a) = parse_item(input).map_err(|e| e.to_string())?;

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
    fn packet_parser(_: char, input: &str) {
        let packet = parse_packet(input).unwrap();
        assert_eq!(packet.to_string().as_str(), input);
    }

    #[test_case("1", "2", PacketsOrder::Right)]
    #[test_case("2", "1", PacketsOrder::Wrong)]
    #[test_case("1", "1", PacketsOrder::Dunno)]
    #[test_case("[1,1,3,1,1]", "[1,1,5,1,1]", PacketsOrder::Right)]
    #[test_case("[[1],[2,3,4]]", "[[1],4]", PacketsOrder::Right)]
    #[test_case("[9]", "[[8,7,6]]", PacketsOrder::Wrong)]
    #[test_case("[[4,4],4,4]", "[[4,4],4,4,4]", PacketsOrder::Right)]
    #[test_case("[7,7,7,7]", "[7,7,7]", PacketsOrder::Wrong)]
    #[test_case("[]", "[3]", PacketsOrder::Right)]
    #[test_case("[[[]]]", "[[]]", PacketsOrder::Wrong)]
    #[test_case(
        "[1,[2,[3,[4,[5,6,7]]]],8,9]",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]",
        PacketsOrder::Wrong
    )]
    fn order_checker(left: &str, right: &str, order: PacketsOrder) {
        let left = parse_packet(left).unwrap();
        let right = parse_packet(right).unwrap();

        assert_eq!(check_order(left, right), order);
    }

    mod part1 {
        use super::*;

        fn test_helper(signal: Vec<bool>) -> usize {
            signal
                .iter()
                .enumerate()
                .filter_map(|(i, correct)| correct.then(|| i + 1))
                .sum()
        }

        #[test]
        fn example_works() {
            let input = include_str!("../example");

            let signal = parse_signal(input).unwrap();

            let result: usize = test_helper(check_signal(signal));
            assert_eq!(result, 13);
        }

        #[test]
        fn input_works() {
            let input = include_str!("../input");

            let signal = parse_signal(input).unwrap();

            let result: usize = test_helper(check_signal(signal));
            assert_eq!(result, 4809);
        }
    }
}
