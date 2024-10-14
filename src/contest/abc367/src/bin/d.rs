use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    // Cumulative sum from 0 to i (MOD m)
    let mut b: Vec<usize> = vec![0; 2 * n];
    b[0] = a[0];
    for i in 0..(2 * n - 1) {
        b[i + 1] = (b[i] + a[(i + 1) % n]) % m;
    }

    let mut cnt_list: Vec<usize> = vec![0; m];
    for i in 1..n {
        cnt_list[b[i]] += 1;
    }

    let mut ans = 0;
    for i in n..(2 * n) {
        ans += cnt_list[b[i]];
        cnt_list[b[i + 1 - n]] -= 1;
        cnt_list[b[i]] += 1;
    }

    println!("{}", ans);
}
