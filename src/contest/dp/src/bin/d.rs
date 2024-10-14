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
        n: usize, w: usize,
        baggages: [(usize,isize);n]
    }

    let mut dp = vec![-1; w + 1];
    dp[0] = 0;

    for &(weight, value) in baggages.iter() {
        // dpのcopyを用意する
        let mut tmp_dp = dp.clone();
        for i in 0..=w - weight {
            if dp[i] < 0 {
                continue;
            }
            tmp_dp[i + weight] = tmp_dp[i + weight].max(dp[i] + value);
        }

        dp = tmp_dp;
    }

    let res = dp.iter().fold(0, |max, &v| max.max(v));
    println!("{res}");
}
