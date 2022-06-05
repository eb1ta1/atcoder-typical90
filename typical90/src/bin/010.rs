use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [(usize, i32); n],
        q: usize,
        lr: [(usize, usize); q]
    }
    let mut first_sum = vec![0];
    let mut second_sum = vec![0];
    for i in 1..n+1 {
        if cp[i - 1].0 == 1 {
            first_sum.push(first_sum[i - 1] + cp[i - 1].1);
            second_sum.push(second_sum[i - 1]);
        } else {
            first_sum.push(first_sum[i - 1]);
            second_sum.push(second_sum[i - 1] + cp[i - 1].1);
        }
    }
    for (l, r) in lr {
        println!("{} {}", first_sum[r] - first_sum[l - 1], second_sum[r] - second_sum[l - 1]);
    }
}