#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{self, BufRead},
    };

    #[test]
    fn it_works() {
        let file = File::open("input").unwrap();

        let mut r: Vec<usize> = io::BufReader::new(file)
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
        r.sort();

        println!("{}", r.iter().rev().take(3).sum::<usize>());
    }
}
