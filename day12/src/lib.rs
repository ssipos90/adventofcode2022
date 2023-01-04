use std::str::FromStr;

use petgraph::algo::astar;
use petgraph::prelude::DiGraphMap;

type Coord = (usize, usize);

#[derive(Debug)]
pub struct HeightMap {
    pub end: Coord,
    pub start: Coord,
    height_map: Vec<Vec<isize>>,
}

impl HeightMap {
    fn calculate_neighbors(x: usize, y: usize, m: usize, n: usize) -> [Option<Coord>; 4] {
        let mut neighbors: [Option<Coord>; 4] = [None; 4];
        // TODO: don't need to look up since up already looked down and vertices are unidirectional
        if x > 0 {
            neighbors[0] = Some((x - 1, y));
        }
        if x + 1 < m {
            neighbors[1] = Some((x + 1, y));
        }
        if y > 0 {
            neighbors[2] = Some((x, y - 1));
        }
        if y + 1 < n {
            neighbors[3] = Some((x, y + 1));
        }
        neighbors
    }

    // pub fn shortest_paths_costs(&self) -> Option<usize> {
    //     let map = dijkstra(&self.graph, self.start, Some(self.end), |_| 1);

    //     map.get(&self.end).cloned()
    // }

    pub fn shortest_path_up(&self) -> Option<(usize, Vec<Coord>)> {
        astar(
            &self.build_graph(&|height, neighbor_height| height + 1 >= neighbor_height),
            self.start,
            |current| current == self.end,
            |_| 1,
            |_| 1,
        )
    }

    pub fn shortest_path_down(&self) -> Option<(usize, Vec<Coord>)> {
        astar(
            &self.build_graph(&|neighbor_height, height| height >= neighbor_height - 1),
            self.end,
            |(x, y)| self.height_map[x][y] == 0,
            |_| 1,
            |_| 1,
        )
    }

    fn build_graph<C>(&self, f: &C) -> DiGraphMap<Coord, ()>
    where
        C: Fn(isize, isize) -> bool,
    {
        let m = self.height_map.len();
        let n = self.height_map[0].len();

        let graph: Vec<(Coord, Coord)> = self
            .height_map
            .iter()
            .enumerate()
            .flat_map(|(x, line)| {
                line.iter().enumerate().flat_map(move |(y, c)| {
                    let height = *c;
                    // println!("{x} {y}: {height}");
                    Self::calculate_neighbors(x, y, m, n)
                        .iter()
                        .filter_map(|neighbor| *neighbor)
                        .filter_map(move |(x1, y1)| {
                            if f(height, self.height_map[x1][y1]) {
                                Some(((x, y), (x1, y1)))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<(Coord, Coord)>>()
                })
            })
            .collect();

        DiGraphMap::from_edges(graph)
    }
}

impl FromStr for HeightMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start: Option<Coord> = None;
        let mut end: Option<Coord> = None;
        let v: Vec<Vec<isize>> = s
            .lines()
            .enumerate()
            .map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(|(y, c)| match c {
                        'S' => {
                            start = Some((x, y));
                            Ok(0)
                        }
                        'E' => {
                            end = Some((x, y));
                            Ok(25)
                        }
                        c @ 'a'..='z' => Ok(c as isize - 97),
                        _ => Err("Char out of range".to_string()),
                    })
                    .collect()
            })
            .collect::<Result<Vec<Vec<isize>>, String>>()?;

        // for r in v.iter() {
        //     println!(
        //         "{}",
        //         r.iter()
        //             .map(|h| char::from_u32((*h as u32) + 97).unwrap().to_string())
        //             .collect::<String>()
        //     );
        // }

        Ok(Self {
            start: start.ok_or_else(|| "Did not found the start position.".to_string())?,
            end: end.ok_or_else(|| "Did not found the end position.".to_string())?,
            height_map: v,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        //use std::fs::write;

        //use petgraph::dot::{Config, Dot};

        use super::*;
        #[test]
        fn example_works() {
            let input = include_str!("../example");
            let graph = HeightMap::from_str(input).unwrap();
            //write(
            //    "graph",
            //    format!(
            //        "{:?}",
            //        Dot::with_config(&graph.graph, &[Config::EdgeNoLabel])
            //    ),
            //)
            //.unwrap();

            let (cost, _path) = graph.shortest_path_up().unwrap();
            //println!("{:?}", _path);

            assert_eq!(cost, 31);
        }

        #[test]
        fn input_works() {
            let input = include_str!("../input");
            let graph = HeightMap::from_str(input).unwrap();

            let (cost, _path) = graph.shortest_path_up().unwrap();

            assert_eq!(cost, 520);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example_works() {
            let input = include_str!("../example");
            let graph = HeightMap::from_str(input).unwrap();

            let (cost, _path) = graph.shortest_path_down().unwrap();

            assert_eq!(cost, 29);
        }

        #[test]
        fn input_works() {
            let input = include_str!("../input");
            let graph = HeightMap::from_str(input).unwrap();

            let (cost, _path) = graph.shortest_path_down().unwrap();

            assert_eq!(cost, 508);
        }
    }
}
