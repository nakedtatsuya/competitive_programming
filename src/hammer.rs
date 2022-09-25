fn is_exist_between(a: isize, b: isize, c: isize) -> bool {
    let min = std::cmp::min(a, b);
    let max = std::cmp::max(a, b);
    min <= c && c <= max
}

fn hammer() {
    input! {
        x:isize, y:isize, z:isize,
    }
    let mut ans = -1;
    if is_exist_between(0, x, y) {
        if !is_exist_between(0, y, z) {
            ans = z.abs() + (z-x).abs();
        }
    } else {
        ans = x.abs();
    }
}