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

fn main() {
    input! {
        n: usize, d: usize,
        mut a: [usize; n]
    }

    // i番目まで見たときにi mod d の要素がBの条件を満たすための最小コスト，j = 0 は消さない，j=1は消す
    let mut dp = vec![vec![1000_000_000_usize; 2]; 1000_001];
    let mut x = vec![0_usize; 1000_001];

    for &ai in a.iter() {
        x[ai] += 1;
    }

    for i in 0..=1000_000_usize {
        if i < d {
            dp[i][0] = 0;
            dp[i][1] = x[i];
        } else {
            dp[i][0] = dp[i - d][1];
            if x[i - d] == 0 {
                dp[i][0] = dp[i][0].min(dp[i - d][0]);
            }
            dp[i][1] = dp[i - d][0].min(dp[i - d][1]) + x[i];
        }
    }

    let mut ans = 0_usize;

    if d == 0 {
        for i in 0..=1000_000 {
            ans += x[i].max(1) - 1;
        }
    } else {
        for i in 0..d {
            ans += dp[1000_000 - i][0].min(dp[1000_000 - i][1]);
        }
    }

    println!("{}", ans);
}
