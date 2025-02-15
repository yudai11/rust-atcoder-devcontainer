use std::collections::HashSet;

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
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1,Usize1); m]
    }

    let mut ans = 0_usize;
    let mut graph = vec![HashSet::new(); n];
    for &(u, v) in edges.iter() {
        if u == v || graph[u].contains(&v) {
            ans += 1;
        } else {
            graph[u].insert(v);
            graph[v].insert(u);
        }
    }

    println!("{ans}");
}
