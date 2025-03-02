use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1,Usize1, usize); m]
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, c) in edges.iter() {
        graph[u].push((v, c));
        graph[v].push((u, c));
    }

    let dist_0 = dijkstra(&graph, n, 0);
    let dist_n = dijkstra(&graph, n, n - 1);

    let mut ans = vec![0_usize; n];
    for i in 0..n {
        ans[i] = dist_0[i] + dist_n[i];
    }

    println!("{}", ans.iter().join("\n"));
}

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, n: usize, start: usize) -> Vec<usize> {
    let infty: usize = 1000_000_000_000_000_000 as usize;
    // returnするvector
    let mut dist = vec![infty; n];
    // 最大値が先頭に来るpriority queue
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start));
    dist[start] = 0;
    while let Some(p) = queue.pop() {
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
