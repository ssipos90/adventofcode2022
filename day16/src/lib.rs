use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use petgraph::{algo::floyd_warshall, prelude::UnGraphMap};

pub struct WeirdVolcano<'a> {
    graph: UnGraphMap<&'a str, ()>,
    flow_rates: HashMap<&'a str, u32>,
}

impl<'a> WeirdVolcano<'a> {
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    fn parse_line(line: &str) -> IResult<&str, (&str, u32, Vec<&str>)> {
        tuple((
            preceded(tag("Valve "), alpha1),
            preceded(tag(" has flow rate="), nom::character::complete::u32),
            preceded(
                tag("; tunnels lead to valves "),
                separated_list1(tag(", "), alpha1),
            ),
        ))(line)
    }

    pub fn build(s: &'a str) -> Result<Self, String> {
        let mut graph: UnGraphMap<&str, ()> = UnGraphMap::new();
        let mut flow_rates = HashMap::new();

        for line in s.lines() {
            let (_, (room, flow_rate, connections)) =
                Self::parse_line(line).map_err(|e| e.to_string())?;
            flow_rates.insert(room, flow_rate);
            for other in connections {
                graph.add_edge(room, other, ());
            }
        }

        Ok(Self { graph, flow_rates })
    }

    pub fn compress(&self) -> Result<HashMap<(&'a str, &'a str), u32>, String> {
        floyd_warshall(&self.graph, |edge| {
            let flow_rate = self.flow_rates.get(edge.1).unwrap();
            *flow_rate
        })
        .map_err(|_| "Negative cycle".to_string())
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
            let volcano = WeirdVolcano::build(input).unwrap();
            assert_eq!(result, 4);
        }
    }
}
