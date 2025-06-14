use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;
// use superslice::Ext; // for use of lowerbound upperbound method of vetor

// ORパス

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1,Usize1, usize); m]
    }

    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];

    for &(a, b, c) in edges.iter() {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let dist = dijkstra(&graph, n, 0);
    let ans = dist[n - 1]
        .iter()
        .fold(1000_000_000_000_000_000_usize, |min_val, &x| min_val.min(x));
    println!("{}", ans);
}

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, n: usize, start: usize) -> Vec<Vec<usize>> {
    let infty: usize = 1000_000_000_000_000_000 as usize;
    // returnするvector
    let mut dist = vec![vec![]; n];
    for i in 0..n {
        dist[i].push(infty);
    }
    let mut cum_or = vec![0_usize; n];
    // 最大値が先頭に来るpriority queue
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start));
    dist[start].push(0);
    while let Some(p) = queue.pop() {
        for &v in graph[p.1].iter() {
            for i in 0..dist[p.1].len() {
                let to_v = dist[p.1][i] | v.1;
                if cum_or[v.0] | to_v > to_v || cum_or[v.0] == 0 {
                    dist[v.0].push(to_v);
                    cum_or[v.0] |= to_v;
                    queue.push((Reverse(to_v), v.0));
                }
            }
        }
    }
    dist
}
