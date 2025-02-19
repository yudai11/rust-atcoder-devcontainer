use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
use std::collections::BinaryHeap;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
// use proconio::marker::Isize1;
use proconio::marker::Usize1;
use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;
// use superslice::Ext; // for use of lowerbound upperbound method of vetor

fn main() {
    input! {
        n: usize,
        edges: [(usize, usize, Usize1); n-1]
    }

    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    for (i, &(a, b, x)) in edges.iter().enumerate() {
        graph[i].push((i + 1, a));
        graph[i].push((x, b));
    }

    let dist_list = dijkstra(&graph, n, 0);
    println!("{}", dist_list[n - 1]);
}

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, n: usize, start: usize) -> Vec<usize> {
    let infty: usize = 1 << 60;
    // returnするvector
    let mut dist = vec![infty; n];
    // 最大値が先頭に来るpriority queue
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start));
    dist[start] = 0;
    while !queue.is_empty() {
        let p = queue.pop().unwrap();
        for &v in graph[p.1].iter() {
            let to_v = dist[p.1] + v.1 as usize;
            if dist[v.0] > to_v {
                dist[v.0] = to_v;
                queue.push((Reverse(to_v), v.0));
            }
        }
    }
    dist
}
