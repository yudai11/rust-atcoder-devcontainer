use itertools::Itertools;
use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// 優先度付きのque, peek,popで最大値を散り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()で最小値を取れる)
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;

fn main() {
    input! {
        n: usize,
        a: [Usize1;n-1]
    }

    let mut graph = vec![vec![]; n];
    for (i, &ai) in a.iter().enumerate() {
        graph[ai].push(i + 1);
    }

    let mut dp = vec![0_usize; n];
    dfs(0, &graph, &mut dp);

    println!("{}", dp.iter().join(" "));
}

fn dfs(node: usize, graph: &Vec<Vec<usize>>, dp: &mut Vec<usize>) {
    for &next in graph[node].iter() {
        dfs(next, graph, dp);
        dp[node] += dp[next] + 1;
    }
}
