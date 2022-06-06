pub fn abs_diff(x: isize, y: isize) -> isize {
    if x < y {
        y - x
    } else {
        x - y
    }
}
