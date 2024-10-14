use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
use proconio::marker::Usize1;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];

    {
        input! {
            abc: [(Usize1,Usize1,usize);m],
        }
        for i in 0..m {
            let (a, b, c) = abc[i];
            graph[a].push((b, c));
            graph[b].push((a, c));
        }
    }

    let dist_from_1 = dijkstra(&graph, n, 0);
    let dist_from_n = dijkstra(&graph, n, n - 1);

    for k in 0..n {
        println!("{}", dist_from_1[k] + dist_from_n[k]);
    }
}

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, n: usize, start: usize) -> Vec<u64> {
    let infty: u64 = 1 << 60;
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
