use std::io::{stdout, Write};

pub fn range_inclusive(a: usize, b: usize) -> impl Iterator<Item = usize> {
    let x: Box<dyn Iterator<Item = usize>> = if b > a {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    };

    x
}

pub fn print_world(world: &[Vec<usize>]) {
    let mut s = stdout();
    for line in world {
        s.write_all(
            line.iter()
                .map(|&block| match block {
                    0 => '.',
                    1 => '#',
                    2 => 'o',
                    5 => '+',
                    _ => panic!("unknown block."),
                })
                .collect::<String>()
                .as_bytes(),
        )
        .unwrap();
        s.write_all([b'\n'].as_slice()).unwrap();
    }
}
