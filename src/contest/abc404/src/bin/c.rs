use ac_library::Dsu;
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
use proconio::marker::Usize1;
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

    if n != m {
        println!("No");
        return;
    }

    let mut graph = vec![vec![]; n];
    let mut dsu = Dsu::new(n);
    for &(u, v) in edges.iter() {
        dsu.merge(u, v);
        graph[u].push(v);
        graph[v].push(u);
    }

    if dsu.groups().len() != 1 {
        println!("No");
        return;
    }

    for i in 0..n {
        if graph[i].len() != 2 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
