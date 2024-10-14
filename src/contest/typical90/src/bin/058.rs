use std::{process::exit, usize};

use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize, k: u64
    }

    let mut ans = n;
    let mut seen = [false; 100000];
    let mut first_seen: [usize; 100000] = [0; 100000];
    seen[n] = true;
    first_seen[n] = 0;
    let mut cycle: u64 = 0;
    for i in 0..100000 as usize {
        ans = strangely_calculate(ans);
        if (i + 1) as u64 >= k {
            println!("{ans}");
            exit(0);
        }
        if seen[ans] {
            cycle = (i + 1 - first_seen[ans]) as u64;
            break;
        }
        seen[ans] = true;
        first_seen[ans] = i + 1;
    }

    let j = ((k - first_seen[ans] as u64) % cycle) as usize;
    for _ in 0..j {
        ans = strangely_calculate(ans);
    }

    println!("{}", ans);
}

fn strangely_calculate(x: usize) -> usize {
    let mut cp_x = x;
    let mut y: usize = 0;
    for _i in 0..5 {
        y += cp_x % 10;
        cp_x /= 10;
    }

    return (x + y) % 100000;
}
