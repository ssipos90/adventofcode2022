use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded},
    *,
};

#[derive(Clone, Debug, PartialEq)]
pub enum MonkeyTest {
    Mod(u64),
}

impl MonkeyTest {
    pub fn apply(&self, worry: u64) -> bool {
        match self {
            MonkeyTest::Mod(x) => worry % x == 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MonkeyOperation {
    Add(u64),
    Square,
    Mul(u64),
}

impl MonkeyOperation {
    pub fn apply(&self, old: u64) -> u64 {
        match self {
            MonkeyOperation::Add(x) => old + x,
            MonkeyOperation::Square => old * old,
            MonkeyOperation::Mul(x) => old * x,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Monkey {
    pub id: u64,
    pub inspections: usize,
    pub starting_items: Vec<u64>,
    pub operation: MonkeyOperation,
    pub test: MonkeyTest,
    pub test_yay: u64,
    pub test_nay: u64,
}

fn operation(input: &str) -> IResult<&str, MonkeyOperation> {
    let (input, _) = tag("Operation: new = old ")(input)?;
    let (input, operation) = alt((
        tag("* old").map(|_| MonkeyOperation::Square),
        preceded(tag("* "), character::complete::u64).map(MonkeyOperation::Mul),
        preceded(tag("+ "), character::complete::u64).map(MonkeyOperation::Add),
    ))(input)?;

    Ok((input, operation))
}

fn header(input: &str) -> IResult<&str, u64> {
    delimited(tag("Monkey "), character::complete::u64, tag(":"))(input)
}

fn test_parser(input: &str) -> IResult<&str, MonkeyTest> {
    let test_mul_parser =
        preceded(tag("divisible by "), character::complete::u64).map(MonkeyTest::Mod);

    // let test_parser_1 = alt((
    //     test_mul_parser,
    //     some_other_parser
    // ));

    preceded(tag("Test: "), test_mul_parser)(input)
}

fn starting_items_parser(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        tag("Starting items: "),
        separated_list0(tag(", "), character::complete::u64),
    )(input)
}

fn branch_test_parser(input: &str) -> IResult<&str, u64> {
    preceded(
        alt((
            tag("If true: throw to monkey "),
            tag("If false: throw to monkey "),
        )),
        character::complete::u64,
    )(input)
}

pub fn monkey_parser(input: &str) -> IResult<&str, Monkey> {
    // let mut monkeys = vec![];
    let (input, id) = header(input)?;

    let (input, _) = multispace1(input)?;
    let (input, starting_items) = starting_items_parser(input)?;
    let (input, _) = multispace1(input)?;
    let (input, operation) = operation(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test) = test_parser(input)?;

    let (input, _) = multispace1(input)?;
    let (input, test_yay) = branch_test_parser(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test_nay) = branch_test_parser(input)?;

    Ok((
        input,
        Monkey {
            operation,
            test,
            test_yay,
            test_nay,
            id,
            starting_items,
            inspections: 0,
        },
    ))
}

pub fn parse_monkeys(input: &str) -> Result<Vec<Monkey>, String> {
    let (_, v) = many0(delimited(multispace0, monkey_parser, multispace0))(input)
        .map_err(|e| e.to_string())?;

    Ok(v)
}

// fn play_round(monkeys: Vec<Monkey>) -> Vec<Monkey> {
//     let mut transfers = vec![];
// }

pub fn monkey_keep_away<W>(mut monkeys: Vec<Monkey>, worry_fn: W, rounds: usize) -> Vec<Monkey>
where
    W: Fn(u64) -> u64,
{
    let mut transfers: HashMap<u64, Vec<u64>> =
        HashMap::from_iter(monkeys.iter().map(|cm| (cm.id, vec![])));
    for round in 1..=rounds {
        if round % 1000 == 0 || round == 20 {
            println!("\n== After round {round} ==");
        }
        for cm in monkeys.iter_mut() {
            cm.starting_items.append(transfers.get_mut(&cm.id).unwrap());
            cm.inspections += cm.starting_items.len();
            if round % 1000 == 0 || round == 20 || round == 1 {
                println!("Monkey {} inspected items {} times.", cm.id, cm.inspections);
            }
            cm.starting_items.drain(0..).for_each(|old| {
                let new = worry_fn(cm.operation.apply(old));
                let dm = if cm.test.apply(new) {
                    cm.test_yay
                } else {
                    cm.test_nay
                };
                let bag = transfers.get_mut(&dm).unwrap();
                bag.push(new);
            });
        }
    }
    monkeys
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Monkey 1:", 1)]
    #[test_case("Monkey 2:", 2)]
    #[test_case("Monkey 3:", 3)]
    fn header_works(input: &str, expect: u64) {
        let (_, actual) = header(input).unwrap();
        assert_eq!(actual, expect);
    }

    #[test_case("Operation: new = old * 1", 1)]
    #[test_case("Operation: new = old * 19", 19)]
    fn mul_operation_parser_works(input: &str, expected: u64) {
        let (_, actual) = operation(input).unwrap();
        match actual {
            MonkeyOperation::Mul(v) => assert_eq!(v, expected),
            _ => panic!("This should be a multiply"),
        };
    }

    #[test_case("Operation: new = old + 1", 1)]
    #[test_case("Operation: new = old + 19", 19)]
    fn add_operation_parser_works(input: &str, expected: u64) {
        let (_, actual) = operation(input).unwrap();
        match actual {
            MonkeyOperation::Add(v) => assert_eq!(v, expected),
            _ => panic!("This should be a add"),
        };
    }

    #[test_case("Operation: new = old * old")]
    fn square_operation_parser_works(input: &str) {
        let (_, actual) = operation(input).unwrap();
        match actual {
            MonkeyOperation::Square => {}
            _ => panic!("This should be a multiply"),
        };
    }

    #[test_case("Test: divisible by 13", 13)]
    fn test_divisible(input: &str, expected: u64) {
        let (_, actual) = test_parser(input).unwrap();

        match actual {
            MonkeyTest::Mod(v) => assert_eq!(v, expected),
            // _ => panic!("idk what this is.")
        }
    }

    #[test]
    fn parse_single_monkey() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

        let (_, monkey) = monkey_parser(input).unwrap();
        assert_eq!(monkey.id, 0);
        assert_eq!(monkey.starting_items, vec![79, 98]);
        assert_eq!(monkey.operation, MonkeyOperation::Mul(19));
        assert_eq!(monkey.test, MonkeyTest::Mod(23));
        assert_eq!(monkey.test_yay, 2);
        assert_eq!(monkey.test_nay, 3);
    }

    mod part1 {

        use super::*;

        #[test]
        fn example_works() {
            let input = include_str!("../example");
            let v = parse_monkeys(input).unwrap();
            assert_eq!(v.len(), 4);
            let mut v = monkey_keep_away(v, |v| v / 3, 20);
            v.sort_by(|m1, m2| m1.inspections.cmp(&m2.inspections));
            v.reverse();
            let p = v[0..=1].iter().map(|m| m.inspections).product::<usize>();
            assert_eq!(p, 10605);
        }

        #[test]
        fn input_works() {
            let input = include_str!("../input");
            let v = parse_monkeys(input).unwrap();
            assert_eq!(v.len(), 8);
            let mut v = monkey_keep_away(v, |v| v / 3, 20);
            v.sort_by(|m1, m2| m1.inspections.cmp(&m2.inspections));
            v.reverse();
            let p = v[0..=1].iter().map(|m| m.inspections).product::<usize>();
            assert_eq!(p, 110264);
        }
    }

    mod part2 {
        use super::*;
        #[test]
        fn example_works() {
            let input = include_str!("../example");
            let v = parse_monkeys(input).unwrap();
            let magic: u64 = v.iter().map(|m| match m.test {
                MonkeyTest::Mod(x) => x,
            }).product();
            assert_eq!(v.len(), 4);
            let mut v = monkey_keep_away(v, |x| x % magic, 10_000);
            v.sort_by(|m1, m2| m1.inspections.cmp(&m2.inspections));
            v.reverse();
            let p = v[0..=1].iter().map(|m| m.inspections).product::<usize>();
            assert_eq!(p, 2713310158);
        }

        #[test]
        fn input_works() {
            let input = include_str!("../input");
            let v = parse_monkeys(input).unwrap();
            let magic: u64 = v.iter().map(|m| match m.test {
                MonkeyTest::Mod(x) => x,
            }).product();
            assert_eq!(v.len(), 8);
            let mut v = monkey_keep_away(v, |x| x % magic, 10_000);
            v.sort_by(|m1, m2| m1.inspections.cmp(&m2.inspections));
            v.reverse();
            let p = v[0..=1].iter().map(|m| m.inspections).product::<usize>();
            assert_eq!(p, 23612457316);
        }
    }
}
