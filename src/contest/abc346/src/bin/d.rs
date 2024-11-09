use std::usize;

use proconio::{input, marker::Chars};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// 優先度付きのque, peek,popで最大値を散り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()で最小値を取れる)
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        s: Chars,
        c: [usize;n]
    }

    let mut dp = vec![[[usize::MAX; 2]; 2]; n + 1];
    match s[0] {
        '0' => {
            dp[0][0][0] = 0;
            dp[0][1][0] = c[0];
        }
        '1' => {
            dp[0][1][0] = 0;
            dp[0][0][0] = c[0];
        }
        _ => unreachable!(),
    }

    for i in 0..n - 1 {
        match s[i + 1] {
            '0' => {
                dp[i + 1][0][0] = dp[i][1][0];
                dp[i + 1][1][0] = dp[i][0][0] + c[i + 1];
                dp[i + 1][0][1] = dp[i][1][1].min(dp[i][0][0]);
                dp[i + 1][1][1] = dp[i][0][1].min(dp[i][1][0]) + c[i + 1];
            }
            '1' => {
                dp[i + 1][1][0] = dp[i][0][0];
                dp[i + 1][0][0] = dp[i][1][0] + c[i + 1];
                dp[i + 1][1][1] = dp[i][0][1].min(dp[i][1][0]);
                dp[i + 1][0][1] = dp[i][1][1].min(dp[i][0][0]) + c[i + 1];
            }
            _ => unreachable!(),
        }
    }

    let mut ans = usize::MAX;
    for i in 0..=1 {
        ans = ans.min(dp[n - 1][i][1]);
    }

    println!("{ans}");
}
