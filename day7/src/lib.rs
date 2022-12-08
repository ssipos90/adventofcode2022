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
pub struct DS<'a>(&'a str, u32, Vec<DS<'a>>);

pub fn flatten_tree<'a>(tree: &[TreeEntry<'a>]) -> Vec<(&'a str, u32)> {
    let DS(name, size, rest) = flatten_tree_helper("/", tree);
    rest.iter()
        .flat_map(|ds| {
            let mut v = vec![(name, size)];
            v.append(&mut ds.2.iter().map(|ds| {
                (ds.0, ds.1)
            }).collect());
            v
        })
        .collect()
}
fn flatten_tree_helper<'a>(name: &'a str, children: &[TreeEntry<'a>]) -> DS<'a> {
    let (size, rest) = children.iter().fold((0, vec![]), |mut acc, child| match child {
        TreeEntry::File(_, size) => (acc.0 + *size, acc.1),
        TreeEntry::Dir(name, children) => {
            let ds = flatten_tree_helper(name, children);
            acc.1.push(ds);
            (acc.0, acc.1)
        }
    });
    DS(name, size, rest)
}

// fn flatten_tree_helper<'a>(name: &'a str, children: &[TreeEntry<'a>]) -> DS<'a> {
//     let (size, rest) = children.iter().fold((0, vec![]), |acc, child| match child {
//         TreeEntry::File(_, size) => (acc.0 + *size, acc.1),
//         TreeEntry::Dir(name, children) => {
//             let ds = flatten_tree_helper(name, children);
//             acc.1.push(ds);
//             (acc.0, acc.1)
//         }
//     });
//     DS(name, size, rest)
// }

#[cfg(test)]
mod tests {
    use super::*;
    mod part1 {
        use super::*;
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
            let ops = parse_output(input);
            let tree = build_tree(&ops);
            let sizes = flatten_tree(&tree);
            let under_10k = sizes
                .iter()
                .filter_map(|&dir| if dir.1 <= 100000 { Some(dir.1) } else { None })
                .collect::<Vec<_>>();
            println!("{:?}", under_10k);
            let under_10k_sum: u32 = under_10k.iter().sum();
            assert_eq!(under_10k_sum, 95437);
        }
    }
}
