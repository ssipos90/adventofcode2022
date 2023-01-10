use crate::{
    parser::{Pair, World},
    util::range_inclusive,
};

pub fn build_world(source: Pair, obstacles: &[Vec<Pair>]) -> (Pair, World) {
    let (min_width, max_width, depth) = get_world_bounds(obstacles);

    let left_offset = min_width - 1;
    let source = (source.0 - left_offset, source.1);
    let width = max_width - min_width + 2;

    let mut world = vec![vec![0; width + 1]; depth + 1];
    world[source.1][source.0] = 5;

    for obstacle in obstacles {
        for segment in obstacle.windows(2) {
            // obstacle ends will overlap but this ain't rocket science
            if let [(x1, y1), (x2, y2)] = segment {
                // carthesian product
                for y in range_inclusive(*x1, *x2) {
                    for x in range_inclusive(*y1, *y2) {
                        world[x][y - left_offset] = 1;
                    }
                }
            } else {
                panic!("windows should work on obstacle segments");
            }
        }
    }

    (source, world)
}

// pub fn build_capped_world(source: Pair, obstacles: &[Vec<Pair>]) -> (Pair, World) {
//     let (min_width, max_width, depth) = get_world_bounds(obstacles);
//     let depth = depth + 2;
// 
//     let s = depth - 1;
// 
//     let bottom = vec![(depth, depth / 2, 2)];
// }

fn get_world_bounds(obstacles: &[Vec<Pair>]) -> (usize, usize, usize) {
    obstacles.iter().fold((usize::MAX, 0, 0), |acc, obstacle| {
        obstacle
            .iter()
            .fold(acc, |(min_width, max_width, max_depth), (y, x)| {
                (min_width.min(*y), max_width.max(*y), max_depth.max(*x))
            })
    })
}

pub fn simulate_world(source: Pair, mut world: World) -> (usize, World) {
    // since only one block of sand drops at a time, we can put it in it's own loop
    let rules: [(isize, isize); 3] = [(1, 0), (1, -1), (1, 1)];
    let mut count = 0;
    let mut sanity = 0;
    let max_height = world.len() as isize;
    loop {
        let mut block = (source.1 as isize, source.0 as isize);
        while block.0 < max_height - 1 {
            let next_block = rules.iter().find_map(|(dx, dy)| {
                let x = block.0 + *dx;
                let y = block.1 + *dy;

                if x < 0 || y < 0 {
                    return None;
                }

                match world[x as usize][y as usize] {
                    0 => Some((x, y)),
                    _ => None,
                }
            });
            sanity += 1;
            if sanity > 1000000 {
                panic!("Waa");
            }

            match next_block {
                Some((x, y)) => {
                    block = (x, y);
                }
                None => break,
            }
        }
        // print_world(&world);

        if block.0 == max_height - 1 {
            break;
        }
        count += 1;
        world[block.0 as usize][block.1 as usize] = 2;
        if block.0 as usize == source.1 {
            break;
        }
    }

    (count, world)
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_input;

    use super::*;

    mod part1 {
        use crate::util::print_world;

        use super::*;
        #[test]
        fn example_works() {
            let input = include_str!("../example");
            let obstacles = parse_input(input).unwrap();
            let (source, world) = build_world((500, 0), &obstacles);
            let (blocks_number, _world) = simulate_world(source, world);
            print_world(&_world);

            assert_eq!(blocks_number, 24);
        }

        #[test]
        fn input_works() {
            let input = include_str!("../input");
            let obstacles = parse_input(input).unwrap();
            let (source, world) = build_world((500, 0), &obstacles);
            let (blocks_number, _world) = simulate_world(source, world);

            assert_eq!(blocks_number, 858);
        }
    }
}
