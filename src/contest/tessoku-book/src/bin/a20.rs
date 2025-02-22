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
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;

fn main() {
    input! {
        s: Chars, t: Chars
    }

    let n = (s.len()).min(t.len());

    let mut dp = vec![vec![0_usize; n]; n];

    for i in 1..n {
        dp[0][i] = dp[0][i - 1];
        dp[i][0] = dp[i - 1][0];
        for j in 1..i {
            dp[j][i] = if s[j] == t[i] {
                (dp[j - 1][i - 1] + 1).max(dp[j - 1][i].max(dp[j][i - 1]))
            } else {
                dp[j - 1][i].max(dp[j][i - 1])
            };
            dp[i][j] = if s[i] == t[j] {
                (dp[i - 1][j - 1] + 1).max(dp[i - 1][j].max(dp[i][j - 1]))
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            };
        }
        dp[i][i] = if s[i] == t[i] {
            (dp[i - 1][i - 1] + 1).max(dp[i - 1][i].max(dp[i][i - 1]))
        } else {
            dp[i - 1][i].max(dp[i][i - 1])
        };
    }
    println!("{}", dp[n - 1][n - 1])
}
