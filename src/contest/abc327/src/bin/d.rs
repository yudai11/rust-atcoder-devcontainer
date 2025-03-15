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

fn main() {
    input! {
        n: usize, m: usize,
        a: [Usize1; m],
        b: [Usize1; m]
    }

    let mut graph = vec![vec![]; n];
    // let mut dsu = Dsu::new(n);
    let mut x = vec![None; n];
    let mut seen = vec![false; n];
    for i in 0..m {
        graph[a[i]].push(b[i]);
        graph[b[i]].push(a[i]);
        if a[i] == b[i] {
            // 自己ループがあるときは2部グラフにならない．
            println!("No");
            return;
        }
        // dsu.merge(a[i], b[i]);
    }

    // let conn_decomp = dsu.groups();

    // 連結成分ごとに2部グラフになるか判定する．=> ならなければ即 No!
    for i in 0..n {
        match x[i] {
            Some(_) => {}
            None => {
                seen.fill(false);
                x[i] = Some(0);
                if !dfs(i, &graph, &mut seen, 0, &mut x) {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes");
}

fn dfs(
    node: usize,
    graph: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    state: usize,
    x: &mut Vec<Option<usize>>,
) -> bool {
    for &next in graph[node].iter() {
        if seen[next] {
            continue;
        }

        match x[next] {
            Some(y) => {
                if y != (state + 1) % 2 {
                    return false;
                }
            }
            None => {
                x[next] = Some((state + 1) % 2);
                seen[next] = true;
                if !dfs(next, graph, seen, (state + 1) % 2, x) {
                    return false;
                }
                seen[next] = false;
            }
        }
    }

    return true;
}
