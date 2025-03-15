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
        n: usize, k: usize,
        edges: [(Usize1,Usize1); n * k - 1]
    }

    if k == 1 {
        println!("Yes");
        return;
    }

    let mut graph = vec![vec![]; n * k];
    for &(u, v) in edges.iter() {
        graph[u].push(v);
        graph[v].push(u);
    }

    // 自分自身とその子を合わせて作れるパスの長さの最大値 (quasi)
    let mut dp = vec![1_usize; n * k];

    dfs(0, None, &graph, &mut dp, k);

    let mut num_k_path = 0_usize;
    for i in 0..n * k {
        if dp[i] == k {
            num_k_path += 1;
        } else if dp[i] > k {
            println!("No");
            return;
        }
    }

    if num_k_path == n {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn dfs(node: usize, parent: Option<usize>, graph: &Vec<Vec<usize>>, dp: &mut Vec<usize>, k: usize) {
    match parent {
        Some(p) => {
            let mut max_val = 0_usize;
            let mut second_max = 0_usize;
            let mut num_small_child = 0_usize;
            for &next in graph[node].iter() {
                if next == p {
                    continue;
                }
                dfs(next, Some(node), graph, dp, k);
                // 端からパスを作る/ big child
                if dp[next] >= k {
                    continue;
                }
                num_small_child += 1;
                if dp[next] > max_val {
                    second_max = max_val;
                    max_val = dp[next];
                } else if dp[next] > second_max {
                    second_max = dp[next];
                }
            }

            if second_max + max_val + 1 == k {
                dp[node] = k;
                if num_small_child > 2 {
                    dp[node] = k + 3;
                }
            } else {
                dp[node] = max_val + 1;
                if num_small_child > 1 {
                    dp[node] = k + 3;
                }
            }
        }
        None => {
            let mut max_val = 0_usize;
            let mut second_max = 0_usize;
            let mut num_small_child = 0_usize;
            for &next in graph[node].iter() {
                dfs(next, Some(node), graph, dp, k);
                // 端からパスを作る
                if dp[next] >= k {
                    continue;
                }
                num_small_child += 1;
                if dp[next] > max_val {
                    second_max = max_val;
                    max_val = dp[next];
                } else if dp[next] > second_max {
                    second_max = dp[next];
                }
            }

            if second_max + max_val + 1 == k {
                dp[node] = k;
                if num_small_child > 2 {
                    dp[node] = k + 3;
                }
            } else {
                dp[node] = max_val + 1;
                if num_small_child > 1 {
                    dp[node] = k + 3;
                }
            }
        }
    }
}
