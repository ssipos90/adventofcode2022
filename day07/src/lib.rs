#[derive(Debug)]
pub enum TreeEntry<'a> {
    Dir(&'a str, Vec<TreeEntry<'a>>),
    File(&'a str, u32),
}

#[derive(Debug)]
pub enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Ls<'a>>),
}

#[derive(Debug)]
pub enum Ls<'a> {
    File(&'a str, u32),
    Dir(&'a str),
}

#[derive(Debug)]
pub enum Cd<'a> {
    Up,
    Down(&'a str),
}

pub fn parse_output(input: &str) -> Vec<Operation> {
    input
        .split("$ ")
        .skip(1)
        .filter_map(|s| {
            let v: Vec<_> = s.lines().filter(|line| !line.is_empty()).collect();
            let command = *v.first().unwrap();
            let command_pieces = command.splitn(2, ' ').collect::<Vec<_>>();
            match command_pieces[..] {
                ["ls"] => Some(Operation::Ls(
                    v[1..]
                        .iter()
                        .filter_map(|&line| match line.splitn(2, ' ').collect::<Vec<_>>()[..] {
                            ["dir", name] => Some(Ls::Dir(name)),
                            [size, name] => Some(Ls::File(name, size.parse::<u32>().unwrap_or(0))),
                            _ => None,
                        })
                        .collect(),
                )),
                ["cd", ".."] => Some(Operation::Cd(Cd::Up)),
                ["cd", dir] => Some(Operation::Cd(Cd::Down(dir))),
                _ => None,
            }
        })
        .collect::<Vec<_>>()
}

pub fn build_tree<'a>(ops: &[Operation<'a>]) -> Vec<TreeEntry<'a>> {
    let mut ops = ops.iter().rev().collect::<Vec<_>>();
    build_dir(&mut ops)
}

pub fn build_dir<'a>(ops: &mut Vec<&Operation<'a>>) -> Vec<TreeEntry<'a>> {
    let mut current = vec![];
    loop {
        match ops.pop() {
            None | Some(Operation::Cd(Cd::Up)) => return current,
            Some(Operation::Cd(Cd::Down(dir))) => current.push(TreeEntry::Dir(dir, build_dir(ops))),
            Some(Operation::Ls(items)) => current.append(
                &mut items
                    .iter()
                    .filter_map(|item| match item {
                        Ls::File(name, size) => Some(TreeEntry::File(name, *size)),
                        _ => None, // CD down handles this
                    })
                    .collect(),
            ),
        };
    }
}

#[derive(Debug)]
pub struct DirStats<'a>(&'a str, u32, Vec<DirStats<'a>>);

pub fn flatten_tree<'a>(tree: &[TreeEntry<'a>]) -> Vec<(&'a str, u32)> {
    let DirStats(_name, _size, rest) = walk_tree("/", tree);
    rest.iter().flat_map(fl).collect()
}

fn fl<'a>(ds: &DirStats<'a>) -> Vec<(&'a str, u32)> {
    let mut v = vec![(ds.0, ds.1)];
    v.append(&mut ds.2.iter().flat_map(fl).collect());
    v
}

fn walk_tree<'a>(name: &'a str, children: &[TreeEntry<'a>]) -> DirStats<'a> {
    let (size, rest) = children
        .iter()
        .fold((0, vec![]), |mut acc, child| match child {
            TreeEntry::File(_, size) => (acc.0 + *size, acc.1),
            TreeEntry::Dir(name, children) => {
                let ds = walk_tree(name, children);
                let ds_size = ds.1;
                acc.1.push(ds);
                (acc.0 + ds_size, acc.1)
            }
        });
    DirStats(name, size, rest)
}

#[cfg(test)]
mod tests {
    use super::*;
    mod part1 {
        use super::*;
        use std::fs::read_to_string;

        fn test_helper(input: &str) -> u32 {
            let ops = parse_output(input);
            let tree = build_tree(&ops);
            let sizes = flatten_tree(&tree);
            let under_10k = sizes
                .iter()
                .filter_map(|&dir| if dir.1 <= 100000 { Some(dir.1) } else { None })
                .collect::<Vec<_>>();
            under_10k.iter().sum()
        }

        #[test]
        fn example_works() {
            let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
            let under_10k_sum = test_helper(input);
            assert_eq!(under_10k_sum, 95437);
        }

        #[test]
        fn input_works() {
            let input = read_to_string("input").unwrap();

            let under_10k_sum = test_helper(&input);
            assert_eq!(under_10k_sum, 1315285);
        }
    }

    mod part2 {
        use std::fs::read_to_string;

        use super::*;

        fn test_helper(input: &str, disk_size: u32, update_size: u32) -> Option<u32> {
            let ops = parse_output(input);
            let tree = build_tree(&ops);
            let sizes = flatten_tree(&tree);
            let free = disk_size - sizes[0].1;
            if free > update_size {
                None
            } else {
                let needed = update_size - free;
                sizes
                    .iter()
                    .filter_map(|&dir| if dir.1 >= needed { Some(dir.1) } else { None })
                    .min()
            }
        }

        #[test]
        fn example_works() {
            let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
            let size = test_helper(input, 70_000_000, 30_000_000).unwrap();

            assert_eq!(size, 24933642);
        }

        #[test]
        fn input_works() {
            let input = read_to_string("input").unwrap();

            let size = test_helper(&input, 70_000_000, 30_000_000).unwrap();

            assert_eq!(size, 9847279);
        }
    }
}
