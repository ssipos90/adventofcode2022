use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input").unwrap();

    let r: Vec<usize> = io::BufReader::new(file)
        .lines()
        .fold(vec![0], |mut acc, item| {
            let line = item.unwrap();
            if line.is_empty() {
                acc.push(0);
            } else {
                let a = acc.last_mut().unwrap();
                *a += line.parse::<usize>().unwrap();
            }
            acc
        });
    println!("{:?}", r.iter().max());
}
