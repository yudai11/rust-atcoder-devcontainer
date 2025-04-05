use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;
// use superslice::Ext; // for use of lowerbound upperbound method of vetor

fn main() {
    input! {
        n: usize,
        c: [Usize1; n],
        x: [usize; n]
    }

    // i~jまで色を揃えたときのコスト(区間dp)
    let mut dp = vec![vec![1000_000_000_usize; 2 * n]; 2 * n];
    for i in 0..2 * n {
        dp[i][i] = x[c[i]] + 1;
    }

    // 区間幅が小さい順に処理
    for len in 0..2 * n {
        for left in 0..2 * n - len {
            let right = left + len;
            if left > 0 {
                let score_l = if right >= scores[left - 1].0 && left <= scores[left - 1].0 {
                    scores[left - 1].1
                } else {
                    0
                };
                dp[left][right] = dp[left][right].min(dp[left - 1][right] + score_l);
            }
            if right < n - 1 {
                let score_r = if right >= scores[right + 1].0 && left <= scores[right + 1].0 {
                    scores[right + 1].1
                } else {
                    0
                };
                dp[left][right] = dp[left][right].max(dp[left][right + 1] + score_r);
            }
        }
    }
}
