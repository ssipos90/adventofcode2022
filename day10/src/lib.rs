#[derive(Clone, Copy, Debug, PartialEq)]
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

struct Registry {
    x: i32,
}

pub fn interpretor<E: std::fmt::Display>(
    mut lines: impl Iterator<Item = Result<String, E>>,
) -> Result<i32, String> {
    let mut registry = Registry { x: 1 };
    let mut last_instruction: Option<Instruction> = None;
    let mut signal: i32 = 0;
    for cycle in 1.. {
        // println!("\ncycle: {cycle}");
        if cycle == 20 || (cycle + 20) % 40 == 0 {
            // println!(" => calculating signal: {signal}");
            signal += cycle * registry.x;
        }
        match last_instruction {
            Some(instruction) => {
                if let Instruction::AddX(x) = instruction {
                    // println!("adding {x}");
                    registry.x += x;
                };
                last_instruction = None;
            }
            None => {
                match lines.next() {
                    Some(Ok(line)) => match parse_instruction(line.as_str())? {
                        Instruction::Noop => {
                            // println!("noop");
                        }
                        instruction => {
                            // println!("setting multi cycle instruction");
                            last_instruction = Some(instruction);
                        }
                    },
                    Some(Err(e)) => return Err(e.to_string()),
                    None => {
                        // println!("finished all instructions");
                        break;
                    }
                }
            }
        };
        // println!("registry value: {}", registry.x);
    }

    Ok(signal)
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

    mod part1 {
        use std::{
            fs::File,
            io::{BufRead, BufReader},
        };

        use super::*;
        #[test]
        fn example_works() {
            let f = File::open("example").unwrap();
            let buf = BufReader::new(f);

            let result = interpretor(buf.lines()).unwrap();
            assert_eq!(result, 13140)
        }
        #[test]
        fn input_works() {
            let f = File::open("input").unwrap();
            let buf = BufReader::new(f);

            let result = interpretor(buf.lines()).unwrap();
            assert_eq!(result, 15120)
        }
    }
}
