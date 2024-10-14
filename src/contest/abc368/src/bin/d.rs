use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use petgraph::unionfind::UnionFind;
use proconio::marker::Usize1;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        _ab: [(Usize1, Usize1); n-1],
        v: [Usize1; k],
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];

    for i in 0..n - 1 {
        let (a, b): (usize, usize) = _ab[i];
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut seen = vec![false; n];
    let mut when_seen = vec![0; n];
    let mut cnt_v_component: usize = 0;
    let set_v: HashSet<usize> = v.into_iter().collect();

    dfs(0, &graph, &mut seen, 0, &mut cnt_v_component, false);
}

fn dfs(
    node: usize,
    graph: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    turn: usize,
    cnt: &mut usize,
    seen_v_comp: bool,
) -> () {
    seen[node] = true;
    for &next in graph[node].iter() {
        if !seen[next] {
            dfs(next, &graph, seen, turn + 1, cnt, seen_v_comp);
        }
    }

    // turn
}
