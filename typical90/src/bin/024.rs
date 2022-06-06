use proconio::{fastout, input};
use typical90::abs_diff;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: isize,
        a: [usize; n],
        b: [usize; n]
    }
    let mut diff_sum: isize = 0;
    for i in 0..n {
        diff_sum += abs_diff(b[i] as isize, a[i] as isize)
    }
    if k >= diff_sum && (diff_sum - k) % 2 == 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}
