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
// use ac_library::{Additive, Segtree}; // segtree

fn main() {
    input! {
        n: usize,
        edges: [(Usize1,Usize1);n-1]
    }

    let mut ans = n - 2;
    let mut graph = vec![vec![]; n];
    let mut deg: Vec<usize> = vec![0; n];

    for &(ui, vi) in edges.iter() {
        deg[ui] += 1;
        deg[vi] += 1;
        graph[ui].push(vi);
        graph[vi].push(ui);
    }

    for i in 0..n {
        let mut x = deg[i];

        // 葉だけ消す
        let mut y = n;
        for &next in graph[i].iter() {
            y = y.min(deg[next] - 1);
        }
        ans = ans.min(n - 1 - x * y - x);

        // 根も消す
        let mut xx = vec![];
        for &v in graph[i].iter() {
            xx.push(deg[v]);
        }
        xx.sort();
        xx.reverse();
        let mut y = n;
        let mut x = 0;
        for i in 0..xx.len() {
            // 葉が多い順に選ぶ
            x += 1;
            y = y.min(xx[i] - 1);
            ans = ans.min(n - 1 - x * y - x);
        }
    }

    println!("{ans}");
}
