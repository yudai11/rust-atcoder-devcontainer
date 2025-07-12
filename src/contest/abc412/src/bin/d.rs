use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;
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
        n: usize, m: usize,
        edges: [(Usize1,Usize1); m]
    }

    let mut graph = vec![HashSet::new(); n];

    for &(u, v) in edges.iter() {
        graph[u].insert(v);
        graph[v].insert(u);
    }

    let mut ans = 100_usize;

    for p in (1..n).permutations(n - 1) {
        let mut cover = 0_usize;
        if graph[0].contains(&p[0]) {
            cover += 1;
        }
        for i in 0..n - 2 {
            if graph[p[i]].contains(&p[i + 1]) {
                cover += 1;
            }
        }
        if graph[p[n - 2]].contains(&0) {
            cover += 1;
        }

        ans = ans.min(n + m - cover * 2);
    }

    println!("{}", ans);
}
