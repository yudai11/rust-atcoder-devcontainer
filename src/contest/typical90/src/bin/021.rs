// use petgraph::graph;
use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab : [(Usize1, Usize1); m]
    }

    let mut graph = vec![vec![]; n];
    let mut graph_rev = vec![vec![]; n];

    for i in 0..m {
        graph[ab[i].0].push(ab[i].1);
        graph_rev[ab[i].1].push(ab[i].0);
    }
    let mut stop_ord = vec![];
    let mut seen = vec![false; n];

    for i in 0..n {
        if !seen[i] {
            dfs(i, &graph, &mut seen, &mut stop_ord);
        }
    }

    stop_ord.reverse();
    seen.fill(false);
    // for i in 0..n {
    //     seen[i] = false;
    // }
    let mut ans: u64 = 0;

    for &i in stop_ord.iter() {
        if !seen[i] {
            let cnt: u64 = dfs_rev(i, &graph_rev, &mut seen);
            ans += (cnt * (cnt - 1) / 2) as u64;
        }
    }

    println!("{}", ans);
}

fn dfs(
    node: usize,
    graph: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    stop_ord: &mut Vec<usize>,
) -> () {
    seen[node] = true;
    for &v in graph[node].iter() {
        if !seen[v] {
            dfs(v, &graph, seen, stop_ord)
        }
    }
    stop_ord.push(node);
}

fn dfs_rev(node: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>) -> u64 {
    seen[node] = true;
    let mut res: u64 = 1;
    for &v in graph[node].iter() {
        if !seen[v] {
            res += dfs_rev(v, &graph, seen);
        }
    }

    res
}
