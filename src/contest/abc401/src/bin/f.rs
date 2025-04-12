use itertools::Itertools;
use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;
// use superslice::Ext; // for use of lowerbound upperbound method of vetor

// 全方位木DP
fn main() {
    input! {
        n1: usize,
        edges1: [(Usize1,Usize1);n1-1],
        n2: usize,
        edges2: [(Usize1,Usize1);n2-1]
    }

    let mut graph1 = vec![vec![]; n1];
    for &(u, v) in edges1.iter() {
        graph1[u].push(v);
        graph1[v].push(u);
    }

    // 最遠点との距離/ 木の直径
    let mut dp1 = vec![0_usize; n1];
    dfs1(0, None, &graph1, &mut dp1);
    dfs2(0, None, &graph1, &mut dp1, 0);

    let mut graph2 = vec![vec![]; n2];
    for &(u, v) in edges2.iter() {
        graph2[u].push(v);
        graph2[v].push(u);
    }

    // 最遠点との距離
    let mut dp2 = vec![0_usize; n2];
    dfs1(0, None, &graph2, &mut dp2);
    dfs2(0, None, &graph2, &mut dp2, 0);

    dp1.sort();
    dp2.sort();

    let min_val = dp1[n1 - 1].max(dp2[n2 - 1]);

    let ans1 = dp1.iter().fold(0_usize, |res, &x| res + x) * n2;
    let ans2 = dp2.iter().fold(0_usize, |res, &x| res + x) * n1;
    let ans = ans1 + ans2 + n1 * n2;

    println!("{}", ans);
}

fn dfs1(node: usize, parent: Option<usize>, graph: &Vec<Vec<usize>>, dp: &mut Vec<usize>) {
    match parent {
        Some(p) => {
            for &next in graph[node].iter() {
                if next == p {
                    continue;
                }
                dfs1(next, Some(node), graph, dp);
                dp[node] = dp[node].max(dp[next] + 1);
            }
        }
        None => {
            for &next in graph[node].iter() {
                dfs1(next, Some(node), graph, dp);
                dp[node] = dp[node].max(dp[next] + 1);
            }
        }
    }
}

fn dfs2(
    node: usize,
    parent: Option<usize>,
    graph: &Vec<Vec<usize>>,
    dp: &mut Vec<usize>,
    y: usize,
) {
    dp[node] = dp[node].max(y);
    let mut q = vec![];
    q.push(0);
    q.push(y);
    for &next in graph[node].iter() {
        if Some(next) != parent {
            q.push(dp[next] + 1);
        }
    }
    q.sort();
    q.reverse();
    // dp[node] = dp[node].max(q[0] + 1);
    for &next in graph[node].iter() {
        if Some(next) == parent {
            continue;
        }
        if q[0] == dp[next] + 1 {
            dfs2(next, Some(node), graph, dp, q[1] + 1);
        } else {
            dfs2(next, Some(node), graph, dp, q[0] + 1);
        }
    }
}
