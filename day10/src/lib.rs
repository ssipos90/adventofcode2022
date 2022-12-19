#[derive(Debug, PartialEq)]
pub enum Instruction {
    Noop,
    AddX(i32),
}

pub fn parse_instruction(line: &str) -> Result<Instruction, String> {
    match line.splitn(2, ' ').collect::<Vec<_>>()[..] {
        ["noop"] => Ok(Instruction::Noop),
        ["addx", x] => Ok(Instruction::AddX(
            x.parse::<i32>().map_err(|e| e.to_string())?,
        )),
        _ => Err("Unknown pattern".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::{Instruction as I, *};
    use test_case::test_case;

    #[test_case("noop", I::Noop)]
    #[test_case("addx 5", I::AddX(5))]
    #[test_case("addx -1",I::AddX(-1))]
    fn instruction_parser_successes(line: &str, expect: I) {
        assert_eq!(parse_instruction(line).unwrap(), expect);
    }

    #[test_case("noop something")]
    #[test_case("addx")]
    #[test_case("addx abc")]
    #[test_case("addx abc 123")]
    #[test_case("addx 123 123")]
    #[test_case("addx abc asd")]
    fn instruction_parser_failures(line: &str) {
        assert!(parse_instruction(line).is_err());
    }

    // mod part1 {
    //     use super::*;
    //     #[test]
    //     fn example_works() {
    //         let input = std::fs::read_to_string("example").unwrap();
    //     }
    // }
}
