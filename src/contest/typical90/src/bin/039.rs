use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
         n: usize,
         edges: [(Usize1,Usize1);n-1]
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &edges {
        graph[u].push((v, 1));
        graph[v].push((u, 1));
    }

    let mut ans: u64 = 0;

    for i in 0..n {
        let dist_list = dijkstra(&graph, n, i);
        for j in (i + 1)..n {
            ans += dist_list[j];
        }
    }

    println!("{}", ans);
}

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, n: usize, start: usize) -> Vec<u64> {
    let infty: u64 = 1 << 60;
    // let n = graph.len();
    // returnするvector
    let mut dist = vec![infty; n];
    // 最大値が先頭に来るpriority queue
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start));
    dist[start] = 0;
    while !queue.is_empty() {
        let p = queue.pop().unwrap();
        for &v in graph[p.1].iter() {
            let to_v = dist[p.1] + v.1 as u64;
            if dist[v.0] > to_v {
                dist[v.0] = to_v;
                queue.push((Reverse(to_v), v.0));
            }
        }
    }
    dist
}
