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
        a: [u64; n]
    }

    let mut dp: Vec<Vec<u128>> = vec![vec![0; 2]; n + 1];

    dp[1][1] = a[0] as u128;
    // dp[i][0]はi番目のモンスターを処理したときに倒した数が2の倍数
    for i in 2..=n {
        dp[i][0] = (dp[i - 1][1] + (2 * a[i - 1] as u128)).max(dp[i - 1][0]);
        dp[i][1] = (dp[i - 1][0] + a[i - 1] as u128).max(dp[i - 1][1]);
    }

    println!("{}", dp[n][0].max(dp[n][1]));
}
