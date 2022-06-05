use proconio::{fastout, input};
fn gcd(x: u64, y: u64) -> u64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}
#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64
    }
    let max = gcd(gcd(a, b), gcd(b, c));
    println!("{}", (a / max + b / max + c / max) - 3);
}
