use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
use std::collections::{BTreeSet, HashSet};

fn main() {
    input! {
        n: usize, q: usize,
    }

    let mut graph = vec![vec![]; n];
    let mut seen = HashSet::new();
    for i in 0..n {
        graph[i].push(i);
        seen.insert((i, i));
    }

    for _i in 0..q {
        input! {
            a: u8
        }
        if a == 1 {
            input! {
                (u, v) : (Usize1, Usize1)
            }
            if seen.contains(&(u, v)) || seen.contains(&(v, u)) {
                continue;
            }
            seen.insert((u, v));
            graph[u].push(v);
            graph[v].push(u);
        } else {
            input! {
                (v, k) : (Usize1, usize)
            }
            let len = graph[v].len();
            if len < k {
                println!("-1");
            } else {
                graph[v].sort_by(|a, b| b.cmp(&a));
                println!("{}", graph[v][k - 1] + 1);
            }
        }
    }
}
