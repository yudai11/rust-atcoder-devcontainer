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
// use ac_library::{Additive, Segtree}; // segtree

fn main() {
    input! {
        n: usize,
        _a: Chars
    }

    let mut a = vec![];
    let mut m = _a.len();

    for i in 0..m {
        a.push(_a[i] as u8 - '0' as u8);
    }

    let mut cost_change: Vec<Vec<(u8, usize)>> = vec![vec![]; n + 1];
    for i in 0..m {
        cost_change[0].push((a[i], 1));
    }
    for i in 1..=n {
        m /= 3;
        for j in 0..m {
            let mut is_approval = 0_u8;
            let mut cost = 1000_000_000_000_000 as usize;
            let x = cost_change[i - 1][3 * j].0
                + cost_change[i - 1][3 * j + 1].0
                + cost_change[i - 1][3 * j + 2].0;
            if x == 0 {
                cost = cost_change[i - 1][3 * j..=3 * j + 2]
                    .iter()
                    .fold(0_usize, |sum, &x| sum + x.1)
                    - cost_change[i - 1][3 * j..=3 * j + 2]
                        .iter()
                        .fold(0_usize, |res, &x| res.max(x.1));
            }
            if x == 1 {
                for k in 0..3 {
                    if cost_change[i - 1][3 * j + k].0 == 0 {
                        cost = cost.min(cost_change[i - 1][3 * j + k].1);
                    }
                }
            }
            if x == 2 {
                is_approval = 1;
                for k in 0..3 {
                    if cost_change[i - 1][3 * j + k].0 == 1 {
                        cost = cost.min(cost_change[i - 1][3 * j + k].1);
                    }
                }
            }
            if x == 3 {
                is_approval = 1;
                cost = cost_change[i - 1][3 * j..=3 * j + 2]
                    .iter()
                    .fold(0_usize, |sum, &x| sum + x.1)
                    - cost_change[i - 1][3 * j..=3 * j + 2]
                        .iter()
                        .fold(0_usize, |res, &x| res.max(x.1));
            }

            cost_change[i].push((is_approval, cost));
        }
    }

    println!("{}", cost_change[n][0].1);
}
