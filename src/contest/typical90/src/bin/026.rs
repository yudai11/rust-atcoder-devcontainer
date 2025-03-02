use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        edges: [(Usize1,Usize1); n-1]
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut seen = vec![false; n];
    let mut list_1: Vec<usize> = vec![];
    let mut list_2 = vec![];
    dfs(0, &graph, &mut seen, &mut list_1, &mut list_2, true, n / 2);

    if list_1.len() == n / 2 {
        println!("{}", list_1.iter().join(" "));
    } else {
        println!("{}", list_2.iter().join(" "));
    }
}

fn dfs(
    node: usize,
    graph: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    list_1: &mut Vec<usize>,
    list_2: &mut Vec<usize>,
    state: bool,
    n: usize,
) {
    seen[node] = true;
    if state && list_1.len() < n {
        list_1.push(node + 1);
    }
    if !state && list_2.len() < n {
        list_2.push(node + 1);
    }
    for &next in graph[node].iter() {
        if !seen[next] {
            dfs(next, graph, seen, list_1, list_2, !state, n);
        }
    }
}
