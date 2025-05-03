use proconio::input;
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

// 期待値DP

fn main() {
    input! {
        n: usize, x: usize,
        mut problems: [(usize,usize,usize); n]
    }

    problems.sort_by(|x, y| x.1.cmp(&y.1));

    // 所持金がi円であり、解いた問題の集合がjであるときの最善行動の期待値
    let mut dp = vec![vec![0.0; 2_usize.pow(n as u32)]; x + 1];

    for i in 1..=x {
        for j in 0..2_usize.pow(n as u32) {
            dp[i][j] = dp[i - 1][j];
        }
        for j in 0..n {
            let (s, c, p) = problems[j];
            let s = s as f64;
            if c > i {
                break;
            }
            for k in 0..2_usize.pow(n as u32) {
                if (k >> j) & 1 != 0 {
                    continue;
                }
                let val = ((dp[i - c][k + (1 << j)] + s) * p as f64
                    + (dp[i - c][k] + s) * (100 - p) as f64) as f64
                    / 100.0;
                if val > dp[i][k + (1 << j)] {
                    dp[i][k + (1 << j)] = val;
                }
            }
        }
    }

    let ans = dp[x].iter().fold(0_f64, |res, &x| res.max(x)) as f64 / 100.0;

    println!("{}", ans);
}
