use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        abc: [[isize;3];n]
    }

    let mut dp = vec![vec![0; 3]; n + 1];
    for i in 0..n {
        for j in 0..3 {
            dp[i + 1][j] = (dp[i][(j + 1) % 3] + abc[i][j]).max(dp[i][(j + 2) % 3] + abc[i][j]);
        }
    }

    let res = dp[n]
        .iter()
        .fold(0, |max, &x| if x > max { x } else { max });
    println!("{res}");
}
