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
        n: usize, t: Usize1,
        edges: [(Usize1,Usize1); n-1]
    }

    let mut relations = vec![vec![]; n];
    for &(x, y) in edges.iter() {
        relations[x].push(y);
        relations[y].push(x);
    }

    let mut dp = vec![0_usize; n];
    dfs(t, None, &relations, &mut dp);

    println!("{}", dp.iter().join(" "));
}

fn dfs(node: usize, parent: Option<usize>, graph: &Vec<Vec<usize>>, dp: &mut Vec<usize>) {
    let mut max_val = 0_usize;
    match parent {
        Some(p) => {
            for &next in graph[node].iter() {
                if next != p {
                    dfs(next, Some(node), graph, dp);
                    max_val = max_val.max(dp[next] + 1);
                }
            }
            dp[node] = max_val;
        }
        None => {
            for &next in graph[node].iter() {
                dfs(next, Some(node), graph, dp);
                max_val = max_val.max(dp[next] + 1);
            }
            dp[node] = max_val;
        }
    }
}
