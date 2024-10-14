use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        mut ab : [(usize, usize); n-1],
    }

    let mut graph_list = vec![vec![]; n];

    for &(u, v) in &ab {
        graph_list[u - 1].push(v - 1);
        graph_list[v - 1].push(u - 1);
    }

    let s = search_furthest(&graph_list, 0).0;
    let ans = search_furthest(&graph_list, s).1 + 1;

    println!("{}", ans);
}

// 一番遠い点とその距離を返す
fn search_furthest(graph_list: &[Vec<usize>], s: usize) -> (usize, u64) {
    let n = graph_list.len();
    let mut seen = vec![false; n];
    let mut dist = vec![0; n];
    let mut queue = VecDeque::new();
    seen[s] = true;
    queue.push_back(s);

    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        for &u in &graph_list[v] {
            if seen[u] {
                continue;
            }
            queue.push_back(u);
            seen[u] = true;
            dist[u] = dist[v] + 1;
        }
    }

    let ans = dist
        .iter()
        .enumerate()
        .max_by_key(|&(_, &v)| v as i32)
        .unwrap();
    return (ans.0, *ans.1);
}
