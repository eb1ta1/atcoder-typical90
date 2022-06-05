#![allow(unused)]
use proconio::{fastout, input};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let rsum: Vec<usize> = (0..h).map(|i| a[i].iter().sum()).collect();
    let csum: Vec<usize> = (0..w).map(|j| a.iter().map(|r| r[j]).sum()).collect();

    for i in 0..h {
        for j in 0..w {
            let t = rsum[i] + csum[j] - a[i][j];
            
            if i != h || j != w {print!("{} ", t);}
            else {print!("{}", t)}
        }
        println!();
    }
}