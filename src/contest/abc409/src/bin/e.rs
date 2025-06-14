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

// 木dp
fn main() {
    input! {
        n: usize,
        mut x: [isize; n],
        edges: [(Usize1,Usize1,isize); n-1]
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, w) in edges.iter() {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    let mut ans = 0_isize;
    // 現在の電価を保存
    let mut dp = x.clone();

    dfs(0, None, &graph, &mut dp, &mut ans);

    println!("{}", ans);
}

fn dfs(
    node: usize,
    parent: Option<usize>,
    graph: &Vec<Vec<(usize, isize)>>,
    dp: &mut Vec<isize>,
    ans: &mut isize,
) {
    match parent {
        Some(p) => {
            for &next in graph[node].iter() {
                if next.0 == p {
                    continue;
                }
                dfs(next.0, Some(node), graph, dp, ans);
                dp[node] += dp[next.0];
                *ans += dp[next.0].abs() * next.1;
                dp[next.0] = 0;
            }
        }
        None => {
            for &next in graph[node].iter() {
                dfs(next.0, Some(node), graph, dp, ans);
                dp[node] += dp[next.0];
                *ans += dp[next.0].abs() * next.1;
                dp[next.0] = 0;
            }
        }
    }
}
