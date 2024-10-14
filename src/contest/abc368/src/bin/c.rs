use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use petgraph::unionfind::UnionFind;

fn main() {
    input! {
        n: usize,
        h: [u64; n],
    }

    let mut t: u128 = 0;

    for i in 0..n {
        t += defeat(&t, &h[i]);
    }

    println!("{}", t);
}

fn defeat(t: &u128, hi: &u64) -> u128 {
    let mut x = ((t + 1) % 3) as usize;
    let mut turn: u128 = 0;
    turn += ((hi / 5) * 3) as u128;
    let mut res = (hi % 5) as isize;
    if res == 0 {
        return turn;
    }
    loop {
        res -= if x % 3 == 0 { 3 } else { 1 };

        turn += 1;
        x += 1;

        if res <= 0 {
            return turn;
        }
    }
}
