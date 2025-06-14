use std::collections::HashSet;

use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;
// use superslice::Ext; // for use of lowerbound upperbound method of vetor

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1,Usize1,usize); m]
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, _w) in edges.iter() {
        graph[u].push(v);
    }

    let mut seen = vec![false; n];
    dfs2(0, &graph, &mut seen);

    // そもそも行けない
    if !seen[n - 1] {
        println!("-1");
        return;
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, w) in edges.iter() {
        graph[u].push((v, w));
    }
    // let mut dp: Vec<Vec<usize>> = vec![vec![]; n];
    // dp[0].push(0_usize);
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    seen.insert((0_usize, 0_usize));

    let mut ans = usize::MAX;
    dfs(0, &graph, &mut seen, n, m, 0_usize, &mut ans);

    println!("{}", ans);
}

fn dfs(
    node: usize,
    graph: &Vec<Vec<(usize, usize)>>,
    seen: &mut HashSet<(usize, usize)>,
    n: usize,
    m: usize,
    val: usize,
    ans: &mut usize,
) {
    if node == n - 1 {
        *ans = (*ans).min(val);
    }

    for &next in graph[node].iter() {
        if seen.contains(&(next.0, val ^ next.1)) {
            continue;
        }
        seen.insert((next.0, val ^ next.1));
        dfs(next.0, graph, seen, n, m, val ^ next.1, ans);
    }
}

fn dfs2(node: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
    seen[node] = true;
    for &next in graph[node].iter() {
        if !seen[next] {
            dfs2(next, graph, seen)
        }
    }
}
