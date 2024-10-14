use std::collections::VecDeque;
// use pathfinding::directed::bfs;
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
        edges: [(Usize1,Usize1,i128);m]
    }

    let mut res: Vec<i128> = vec![-1; n];
    let mut seen: Vec<bool> = vec![false; n];

    let mut graph = vec![vec![]; n];
    // let mut rev_graph = vec![vec![]; n];

    for &(u, v, w) in edges.iter() {
        graph[u].push((v, w));
        graph[v].push((u, -w));
    }

    for i in 0..n {
        if seen[i] {
            continue;
        }

        // bfs
        let mut que = VecDeque::new();
        que.push_back(i);
        res[i] = 0;
        seen[i] = true;

        while !que.is_empty() {
            let v = que.pop_front().unwrap();
            for &(next, w) in &graph[v] {
                if seen[next] {
                    continue;
                } else {
                    seen[next] = true;
                    res[next] = res[v] + w;
                    que.push_back(next);
                }
            }
        }
    }

    for &x in res.iter() {
        print!("{x} ");
    }
}
