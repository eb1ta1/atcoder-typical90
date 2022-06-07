use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
        tx: [(usize, usize);q]
    }
    let mut cards = VecDeque::new();
    for unique_tx in tx {
        let t = unique_tx.0;
        let x = unique_tx.1;
        if t == 1 {
            cards.push_front(x)
        } else if t == 2 {
            cards.push_back(x)
        } else if t == 3 {
            println!("{}", cards[x-1])
        } else {
            println!("Something went wrong")
        }
    }
}
