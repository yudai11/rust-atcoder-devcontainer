use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;
// use superslice::Ext; // for use of lowerbound upperbound method of vetor

const MOD: usize = 1000_000_007;
// 木DP
fn main() {
    input! {
        n: usize,
        // edges: [(Usize1,Usize1); n-1]
    }

    let mut graph = vec![vec![]; n];
    for _i in 0..n - 1 {
        input! {
            (x,y) : (Usize1,Usize1)
        }
        graph[x].push(y);
        graph[y].push(x);
    }

    let mut dp = vec![vec![1000_000_000_000; 2]; n];
    let mut seen = vec![false; n];

    dfs(0, &graph, &mut seen, &mut dp);

    println!("{}", dp[0].iter().sum::<usize>() % MOD)
}

fn dfs(node: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, dp: &mut Vec<Vec<usize>>) {
    seen[node] = true;
    let mut is_leaf = true;
    for &next in graph[node].iter() {
        if !seen[next] {
            is_leaf = false;
            dfs(next, graph, seen, dp)
        }
    }

    if is_leaf {
        dp[node][0] = 1;
        dp[node][1] = 1;
    } else {
        dp[node][0] = 1;
        dp[node][1] = 1;
        for &next in graph[node].iter() {
            if dp[next][0] < 1000_000_000_000 {
                dp[node][0] *= dp[next][0] + dp[next][1];
                dp[node][1] *= dp[next][0];
            }
            dp[node][0] %= MOD;
            dp[node][1] %= MOD;
        }
    }
}
