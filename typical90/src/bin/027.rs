use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        ordered_user_names: [Chars; n]
    }
    let mut exist_user_names = HashSet::new();
    for i in 1..n + 1 {
        if !exist_user_names.contains(&ordered_user_names[i - 1]) {
            println!("{}", i);
            exist_user_names.insert(&ordered_user_names[i - 1]);
        }
    }
}
