use num_integer::div_floor;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize
    }
    if h == 1 || w == 1 {
        println!("{}", h * w)
    } else {
        let led_h;
        let led_w;
        if h % 2 == 0 {
            led_h = h / 2;
        } else {
            led_h = div_floor(h, 2) + 1;
        }

        if w % 2 == 0 {
            led_w = w / 2;
        } else {
            led_w = div_floor(w, 2) + 1;
        }

        println!("{}", led_h * led_w)
    }
}
