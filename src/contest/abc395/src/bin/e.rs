use proconio::{input, marker::Usize1};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize, m: usize, x: usize,
        edges: [(Usize1,Usize1); m]
    }

    let mut graph = vec![vec![]; 2 * n];
    for i in 0..n {
        graph[i].push((i + n, x));
        graph[i + n].push((i, x));
    }
    for &(u, v) in edges.iter() {
        graph[u].push((v, 1));
        graph[v + n].push((u + n, 1));
    }

    let dist = dijkstra(&graph, 2 * n, 0);
    let ans = dist[n - 1].min(dist[2 * n - 1]);
    println!("{}", ans);
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
