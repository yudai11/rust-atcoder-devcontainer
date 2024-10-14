use proconio::input;
use std::collections::VecDeque;
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
        h: [isize; n]
    }

    // let mut h = h.iter().collect::<VecDeque<&isize>>();
    // h.push_front(&0);

    let mut dp = vec![isize::MAX; n + 1];
    dp[1] = 0;
    for i in 1..n {
        for j in (i + 1)..=(i + 2) {
            if j <= n {
                dp[j] = dp[j].min(dp[i] + (h[j - 1] - h[i - 1]).abs())
            }
        }
    }

    println!("{}", dp[n]);
}
