use proconio::{input, marker::Usize1};
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
// use ac_library::{Additive, Segtree}; // segtree

fn main() {
    input! {
        n: usize,
        a: [Usize1; n]
    }

    // 連チャンの数を記録
    // dp[i][0]は
    // let mut dp = vec![vec![0; 2]; n + 1];

    let mut ans: isize = 0;

    let mut seen: Vec<isize> = vec![-2; n + 1];

    // 偶数番目始まり
    let mut l: isize = 0;
    for i in 0..n - 1 {
        if i % 2 == 0 && i + 1 < n {
            if a[i] != a[i + 1] {
                l = i as isize + 2;
            } else {
                l = l.max(seen[a[i]] + 2);
            }
            ans = ans.max(i as isize + 2 - l);
            seen[a[i]] = i as isize;
        }
    }

    let mut seen = vec![-2; n];

    // 奇数番目始まり
    l = 1;
    for i in 1..n - 1 {
        if i % 2 == 1 && i + 1 < n {
            if a[i] != a[i + 1] {
                l = i as isize + 2;
            } else {
                l = l.max(seen[a[i]] + 2);
            }
            ans = ans.max(i as isize + 2 - l);
            seen[a[i]] = i as isize;
        }
    }

    println!("{ans}");
}
