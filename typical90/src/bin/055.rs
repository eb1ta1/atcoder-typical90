use proconio::{fastout, input};
#[fastout]
//TODO 再帰関数を使った再実装を行う
fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n]
    }
    let mut cnt = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                for l in k + 1..n {
                    for m in l + 1..n {
                        if a[i] % p * a[j] % p * a[k] % p * a[l] % p * a[m] % p == q {
                            cnt += 1
                        }
                    }
                }
            }
        }
    }
    println!("{}", cnt)
}
