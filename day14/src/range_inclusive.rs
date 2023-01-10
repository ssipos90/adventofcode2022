pub fn range_inclusive(a: usize, b: usize) -> impl Iterator<Item = usize> {
    let x: Box<dyn Iterator<Item = usize>> = if b > a {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    };

    x
}
