use std::str::FromStr;

type Coord = (usize, usize);

#[derive(Debug)]
pub struct HeightMap {
    pub map: Vec<Vec<usize>>,
    pub start: Coord,
    pub end: Coord,
}

impl HeightMap {
    pub fn shortest_distance() -> Result<Vec<Coord>, String> {

        todo!();
    }
}

#[derive(Debug)]
pub enum Alt {
    Start,
    End,
    Rando(usize),
}

impl FromStr for HeightMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start: Option<Coord> = None;
        let mut end: Option<Coord> = None;
        let map: Vec<Vec<usize>> = s
            .lines()
            .enumerate()
            .map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(|(y, c)| match c {
                        'S' => {
                            start = Some((x, y));
                            Ok('a' as usize)
                        }
                        'E' => {
                            end = Some((x, y));
                            Ok('z' as usize)
                        }
                        c @ 'a'..='z' => Ok(c as usize),
                        _ => Err("Char out of range".to_string()),
                    })
                    .collect()
            })
            .collect::<Result<_, String>>()?;
        Ok(Self {
            start: start.ok_or_else(|| "Did not found the start position.".to_string())?,
            end: end.ok_or_else(|| "Did not found the end position.".to_string())?,
            map,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_shortest_path() {
    }

    mod part1 {
        use super::*;
        #[test]
        fn example_works() {
            let input = include_str!("../example");
            let map = HeightMap::from_str(input).unwrap();
            println!("{:?}", map);
        }
    }
}
