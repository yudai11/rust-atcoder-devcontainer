use proconio::input;
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
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    // LIS (最小増加部分列)
    const MAX_NUM: usize = 1000_000_000_000;
    // 0~iまでの部分列の中でiを末尾にする最長の部分列の長さ
    let mut dp = vec![1_usize; n];
    // 長さiの単調増加列の中で末尾が最小なものを記録
    let mut least_ends = vec![MAX_NUM; n + 1];
    // let mut len = 0_usize;
    // least_ends[0] = a[0];
    for i in 0..n {
        let temp = least_ends.lower_bound(&a[i]);
        dp[i] = if temp <= 0 { temp + 1 } else { temp };

        least_ends[dp[i]] = least_ends[dp[i]].min(a[i]);
    }

    let ans = dp.iter().fold(0_usize, |res, &x| res.max(x));
    println!("{ans}");
}
