use std::str::FromStr;

#[derive(Debug)]
pub struct HeightMap {
    pub map: Vec<Vec<u32>>,
    pub start: (u32, u32),
    pub end: (u32, u32),
}

#[derive(Debug)]
pub enum Alt {
    Start,
    End,
    Rando(u32),
}

impl FromStr for HeightMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start: Option<(u32, u32)> = None;
        let mut end: Option<(u32, u32)> = None;
        let map: Vec<Vec<u32>> = s
            .lines()
            .enumerate()
            .map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(|(y, c)| match c {
                        'S' => {
                            start = Some((x as u32, y as u32));
                            Ok('a' as u32)
                        }
                        'E' => {
                            end = Some((x as u32, y as u32));
                            Ok('z' as u32)
                        }
                        c @ 'a'..='z' => Ok(c as u32),
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
