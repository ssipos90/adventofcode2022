type Move = (usize, usize, usize);
type Stacks = Vec<Vec<char>>;

pub fn parse_moves(moves: &str) -> Result<Vec<Move>, String> {
    moves
        .lines()
        .map(parse_move)
        .collect::<Result<Vec<_>, String>>()
}

fn parse_move(line: &str) -> Result<Move, String> {
    let words = line.split(' ').collect::<Vec<_>>();
    let [a, b, c] = match words[..] {
        ["move", count, "from", from, "to", to] => [
            count.parse::<usize>().map_err(|_| "Failed to parse u32.")?,
            from.parse::<usize>().map_err(|_| "Failed to parse u32.")?,
            to.parse::<usize>().map_err(|_| "Failed to parse u32.")?,
        ],
        _ => return Err("Did not match pattern.".to_string()),
    };
    Ok((a, b, c))
}

pub fn parse_stacks(stacks: &str) -> Result<Stacks, String> {
    let v: Vec<_> = stacks.lines().collect();

    let t: Vec<Vec<_>> = v[0..v.len() - 1]
        .iter()
        .map(|&line| parse_stack_line(line))
        .collect();

    let len = t[0].len();
    let mut acc: Stacks = vec![vec![]; len];
    for current in t.iter().rev() {
        if current.len() != len {
            return Err("Length mismatch".to_string());
        }
        current.iter().zip(&mut acc).for_each(|(a, b)| {
            if let Some(a) = a {
                b.push(*a);
            }
        });
    }
    Ok(acc)
}

fn parse_stack_line(line: &str) -> Vec<Option<char>> {
    let chars: Vec<_> = line.chars().collect();
    chars
        .chunks(4)
        .map(|chunk| {
            // only the first 3 chars are relevant
            match &chunk[..3] {
                ['[', c, ']'] => Some(*c),
                _ => None,
            }
        })
        .collect()
}

pub fn parse_input(input: &str) -> Result<(Stacks, Vec<Move>), String> {
    let (stacks, moves) = input
        .split_once("\n\n")
        .ok_or_else(|| "Failed to split by nl nl.".to_string())?;

    Ok((parse_stacks(stacks)?, parse_moves(moves)?))
}

pub fn process_moves_sequentially(stacks: &Stacks, moves: &Vec<Move>) -> Stacks {
    let mut stacks = stacks.clone();
    for (quantity, from, to) in moves {
        let mut tmp = vec![];
        let from = &mut stacks[*from - 1];
        for _ in 0..*quantity {
            if let Some(c) = from.pop() {
                tmp.push(c);
            }
        }
        let to = &mut stacks[*to - 1];
        for c in tmp.iter() {
            to.push(*c);
        }
    }
    stacks
}
pub fn process_moves_in_chunks(stacks: &Stacks, moves: &Vec<Move>) -> Stacks {
    let mut stacks = stacks.clone();
    for (quantity, from, to) in moves {
        let mut tmp = vec![];
        let from = &mut stacks[*from - 1];
        for _ in 0..*quantity {
            if let Some(c) = from.pop() {
                tmp.push(c);
            }
        }
        let to = &mut stacks[*to - 1];
        for c in tmp.iter().rev() {
            to.push(*c);
        }
    }
    stacks
}

#[cfg(test)]
mod tests {
    use super::*;
    mod part1 {
        use std::fs::read_to_string;

        fn peek_stack(stacks: &Stacks) -> String {
            stacks
                .iter()
                .map(|stack| match stack.last() {
                    None => ' ',
                    Some(c) => *c,
                })
                .collect()
        }

        use super::*;
        #[test]
        fn example_works() {
            let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
            let (stacks, moves) = parse_input(input).unwrap();

            let new_stacks = process_moves_sequentially(&stacks, &moves);

            assert_eq!(peek_stack(&new_stacks).as_str(), "CMZ");
        }

        #[test]
        fn input_works() {
            let input = read_to_string("input").unwrap();
            let (stacks, moves) = parse_input(&input).unwrap();

            let new_stacks = process_moves_sequentially(&stacks, &moves);

            assert_eq!(peek_stack(&new_stacks).as_str(), "SHQWSRBDL");
        }
    }

    mod part2 {
        use std::fs::read_to_string;

        fn peek_stack(stacks: &Stacks) -> String {
            stacks
                .iter()
                .map(|stack| match stack.last() {
                    None => ' ',
                    Some(c) => *c,
                })
                .collect()
        }

        use super::*;
        #[test]
        fn example_works() {
            let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
            let (stacks, moves) = parse_input(input).unwrap();

            let new_stacks = process_moves_in_chunks(&stacks, &moves);

            assert_eq!(peek_stack(&new_stacks).as_str(), "MCD");
        }

        #[test]
        fn input_works() {
            let input = read_to_string("input").unwrap();
            let (stacks, moves) = parse_input(&input).unwrap();

            let new_stacks = process_moves_in_chunks(&stacks, &moves);

            assert_eq!(peek_stack(&new_stacks).as_str(), "CDTQZHBRS");
        }
    }
}
