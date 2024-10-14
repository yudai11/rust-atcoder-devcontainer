// use num::pow;
use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        (a, b, c): (u128, usize, u128),
    }

    if a < pow_mine(c, b) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn pow_mine(c: u128, b: usize) -> u128 {
    let mut ans: u128 = 1;

    for i in 0..b {
        ans *= c;
    }

    ans
}
