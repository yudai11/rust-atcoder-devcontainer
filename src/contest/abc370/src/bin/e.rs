use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n]
    }
    let _mod = 998244353;

    let mut b: Vec<i64> = vec![0; n + 1];
    for i in 0..n {
        b[i + 1] = a[i] + b[i];
    }

    let mut dp = vec![0; n + 2];
    dp[0] = 1;
    dp[1] = 1;

    for i in 1..=n {
        for j in 0..i {
            if b[i] - b[j] != k {
                dp[i + 1] += dp[j];
                dp[i + 1] %= _mod;
            }
        }
    }

    println!("{}", dp[n + 1]);
}
