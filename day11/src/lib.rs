use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    multi::separated_list0,
    sequence::{delimited, preceded},
    *,
};

#[derive(Debug, PartialEq)]
pub enum MonkeyTest {
    Mod(u32),
}

#[derive(Debug, PartialEq)]
pub enum MonkeyOperation {
    Add(u32),
    Square,
    Mul(u32),
}

pub struct Monkey {
    pub id: u32,
    pub starting_items: Vec<u32>,
    pub operation: MonkeyOperation,
    pub test: MonkeyTest,
    pub test_yay: u32,
    pub test_nay: u32,
}

fn operation(input: &str) -> IResult<&str, MonkeyOperation> {
    let (input, _) = tag("Operation: new = old ")(input)?;
    let (input, operation) = alt((
        tag("* old").map(|_| MonkeyOperation::Square),
        preceded(tag("* "), character::complete::u32).map(MonkeyOperation::Mul),
        preceded(tag("+ "), character::complete::u32).map(MonkeyOperation::Add),
    ))(input)?;

    Ok((input, operation))
}

fn header(input: &str) -> IResult<&str, u32> {
    delimited(tag("Monkey "), character::complete::u32, tag(":"))(input)
}

fn test_parser(input: &str) -> IResult<&str, MonkeyTest> {
    let test_mul_parser =
        preceded(tag("divisible by "), character::complete::u32).map(MonkeyTest::Mod);

    // let test_parser_1 = alt((
    //     test_mul_parser,
    //     some_other_parser
    // ));

    preceded(tag("Test: "), test_mul_parser)(input)
}

fn starting_items_parser(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(
        tag("Starting items: "),
        separated_list0(tag(", "), character::complete::u32),
    )(input)
}

fn branch_test_parser(input: &str) -> IResult<&str, u32> {
    preceded(
        alt((
            tag("If true: throw to monkey "),
            tag("If false: throw to monkey "),
        )),
        character::complete::u32,
    )(input)
}

// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3
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
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Monkey 1:", 1)]
    #[test_case("Monkey 2:", 2)]
    #[test_case("Monkey 3:", 3)]
    fn header_works(input: &str, expect: u32) {
        let (_, actual) = header(input).unwrap();
        assert_eq!(actual, expect);
    }

    #[test_case("Operation: new = old * 1", 1)]
    #[test_case("Operation: new = old * 19", 19)]
    fn mul_operation_parser_works(input: &str, expected: u32) {
        let (_, actual) = operation(input).unwrap();
        match actual {
            MonkeyOperation::Mul(v) => assert_eq!(v, expected),
            _ => panic!("This should be a multiply"),
        };
    }

    #[test_case("Operation: new = old + 1", 1)]
    #[test_case("Operation: new = old + 19", 19)]
    fn add_operation_parser_works(input: &str, expected: u32) {
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
    fn test_divisible(input: &str, expected: u32) {
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
        use nom::{character::complete::multispace0, multi::many0};

        use super::*;

        #[test]
        fn example_works() {
            let input = include_str!("../example");
            let (_, v) = many0(delimited(multispace0, monkey_parser, multispace0))(input).unwrap();
            assert_eq!(v.len(), 4);
        }
    }
}
