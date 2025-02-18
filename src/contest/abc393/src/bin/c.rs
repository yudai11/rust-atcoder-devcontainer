use proconio::{input, marker::Usize1};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1,Usize1); m]
    }

    let mut ans = 0_usize;
    let mut graph = vec![HashSet::new(); n];
    for &(u, v) in edges.iter() {
        if u == v || graph[u].contains(&v) {
            ans += 1;
        } else {
            graph[u].insert(v);
            graph[v].insert(u);
        }
    }

    println!("{ans}");
}
