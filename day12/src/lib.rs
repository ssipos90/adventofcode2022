use std::str::FromStr;

use petgraph::prelude::UnGraphMap;

type Coord = (usize, usize);

#[derive(Debug)]
pub struct HeightMap {
    pub end: Coord,
    pub map: UnGraphMap<Coord, ()>,
    pub start: Coord,
}

impl HeightMap {
    fn calculate_neighbors(x: usize, y: usize, m: usize, n: usize) -> [Option<Coord>; 4] {
        let mut neighbors: [Option<Coord>; 4] = [None; 4];
        if x > 0 {
            neighbors[0] = Some((x - 1, y));
        }
        if x < m - 1 {
            neighbors[1] = Some((x + 1, y));
        }
        if y > 0 {
            neighbors[2] = Some((x, y - 1));
        }
        if y < n - 1 {
            neighbors[3] = Some((x, y + 1));
        }
        neighbors
    }

    pub fn shortest_distance(&self) -> Result<Vec<Coord>, String> {
        // self.map.
        todo!()
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
        let v: Vec<Vec<usize>> = s
            .lines()
            .enumerate()
            .map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(|(y, c)| match c {
                        'S' => {
                            start = Some((x, y));
                            Ok('a' as usize - 97)
                        }
                        'E' => {
                            end = Some((x, y));
                            Ok('z' as usize - 97)
                        }
                        c @ 'a'..='z' => Ok(c as usize - 97),
                        _ => Err("Char out of range".to_string()),
                    })
                    .collect()
            })
            .collect::<Result<Vec<Vec<usize>>, String>>()?;

        let m = v.len();
        let n = v[0].len();

        let map = v.iter().enumerate().flat_map(|(x, line)| {
            let v = &v;
            line.iter().enumerate().flat_map(move |(y, c)| {
                let height = *c;
                Self::calculate_neighbors(x, y, m, n)
                    .iter()
                    .filter_map(|a| *a)
                    .filter(move |&(x1, y1)| {
                        let neighbor_height = v[x1][y1];
                        height - 1 <= neighbor_height && height + 1 >= neighbor_height
                    })
                    .map(|b| {
                        ((x, y), b)
                    })
                    .collect::<Vec<_>>()
            })
        });

        Ok(Self {
            start: start.ok_or_else(|| "Did not found the start position.".to_string())?,
            end: end.ok_or_else(|| "Did not found the end position.".to_string())?,
            map: UnGraphMap::from_iter(map),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_shortest_path() {}

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
