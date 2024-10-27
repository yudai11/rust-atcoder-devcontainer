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

fn main() {
    input! {
        n: usize, m: usize,
        mut lr: [(usize,usize);n]
    }

    lr.sort_by_key(|&(x, _)| x);

    let mut dr = vec![m; m + 1];

    for &(l, r) in lr.iter() {
        dr[l] = dr[l].min(r - 1);
    }

    for l in 1..=m {
        dr[l] = dr[l].min(dr[l + 1]);
    }

    let mut ans = 0;
    for l in 1..=m {
        ans += dr[l] + 1 - l;
    }

    println!("{ans}")
}
