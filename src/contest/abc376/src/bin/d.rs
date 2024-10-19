use std::collections::VecDeque;

use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1,Usize1);m]
    }

    let mut graph = vec![vec![]; n];
    let mut seen = vec![false; n];
    for &(a, b) in edges.iter() {
        graph[a].push(b);
    }

    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    que.push_back((0, 0));
    seen[0] = true;

    while let Some((v, t)) = que.pop_front() {
        for &next in &graph[v] {
            if next == 0 {
                println!("{}", t + 1);
                return;
            } else if !seen[next] {
                que.push_back((next, t + 1));
                seen[next] = true;
            }
        }
    }

    println!("-1");
}
