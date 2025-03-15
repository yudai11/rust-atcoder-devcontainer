use ac_library::Dsu;
use proconio::{input, marker::Usize1};
use std::usize;
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
    for &(u, v, w) in edges.iter() {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![]; n];
    dp[0].push(0_usize);
    let mut seen = vec![false; n];
    seen[0] = true;

    let mut ans = usize::MAX;
    dfs(0, &graph, &mut seen, &mut dp, n, 0_usize, &mut ans);

    println!("{}", ans);
}

fn dfs(
    node: usize,
    graph: &Vec<Vec<(usize, usize)>>,
    seen: &mut Vec<bool>,
    dp: &mut Vec<Vec<usize>>,
    n: usize,
    val: usize,
    ans: &mut usize,
) {
    if node == n - 1 {
        *ans = (*ans).min(val);
        return;
    }

    for &next in graph[node].iter() {
        if seen[next.0] {
            continue;
        }
        seen[next.0] = true;
        dfs(next.0, graph, seen, dp, n, val ^ next.1, ans);
        seen[next.0] = false;
    }
}
