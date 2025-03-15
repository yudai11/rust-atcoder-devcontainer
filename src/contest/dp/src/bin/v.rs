use itertools::Itertools;
use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;

// 全方位木DP
fn main() {
    input! {
        n: usize, mm: usize,
        edges: [(Usize1,Usize1); n-1]
    }

    let m = 1000_000_007;
    let mut graph = vec![vec![]; n];
    for &(x, y) in edges.iter() {
        graph[x].push(y);
        graph[y].push(x);
    }

    let mut dp: Vec<usize> = vec![1; n];
    // 0を始点にして木DP
    dfs(0, None, &graph, &mut dp, m);

    // 全方位に遷移させる
    dfs2(0, None, &graph, &mut dp, m);

    for i in 0..n {
        dp[i] += m - 1;
        dp[i] %= m;
        dp[i] %= mm;
    }

    println!("{}", dp.iter().join("\n"));
}

// 木DP
fn dfs(node: usize, parent: Option<usize>, graph: &Vec<Vec<usize>>, dp: &mut Vec<usize>, m: usize) {
    match parent {
        Some(p) => {
            for &next in graph[node].iter() {
                if next != p {
                    dfs(next, Some(node), graph, dp, m);
                    dp[node] *= dp[next];
                    dp[node] %= m;
                }
            }
            dp[node] += 1;
            dp[node] %= m;
        }
        None => {
            for &next in graph[node].iter() {
                dfs(next, Some(node), graph, dp, m);
                dp[node] *= dp[next];
                dp[node] %= m;
            }
            dp[node] += 1;
            dp[node] %= m;
        }
    }
}

fn dfs2(
    node: usize,
    parent: Option<usize>,
    graph: &Vec<Vec<usize>>,
    dp: &mut Vec<usize>,
    m: usize,
) {
    match parent {
        Some(p) => {
            // 親ノードを子としたときの処理
            let past = dp[node];
            dp[node] *= (dp[p] * (mod_inv((past + 1) as isize, m as isize) as usize)) % m + 1;
            dp[node] %= m;
            // nodeの子ノードに対する処理
            for &next in graph[node].iter() {
                if next != p {
                    // let past = dp[next];
                    // dp[next] *=
                    //     (dp[node] * (mod_inv((past + 1) as isize, m as isize) as usize) + 1) % m;
                    // dp[next] %= m;
                    // nextがnodeの子であるときはまずnextに遷移してnextの全方位を完成させる．
                    dfs2(next, Some(node), graph, dp, m);
                }
            }
        }
        None => {
            // nextはnodeの子である
            for &next in graph[node].iter() {
                // nextの周りを探索
                dfs2(next, Some(node), graph, dp, m);
            }
        }
    }
}

// fn dfs2(
//     node: usize,
//     parent: Option<usize>,
//     graph: &Vec<Vec<usize>>,
//     dp: &mut Vec<usize>,
//     m: usize,
// ) {
//     match parent {
//         Some(p) => {
//             // 親ノードを子としたときの処理
//             let past = dp[node];
//             dp[node] *= dp[p] + 1;
//             dp[node] %= m;
//             dp[node] *= mod_inv((past + 1) as isize, m as isize) as usize;
//             dp[node] %= m;
//             // nodeの子ノードに対する処理
//             for &next in graph[node].iter() {
//                 if next != p {
//                     // nextがnodeの子であるときはまずnextに遷移してnextの全方位を完成させる．
//                     dfs2(next, Some(node), graph, dp, m);
//                 }
//             }
//         }
//         None => {
//             // nextはnodeの子である
//             for &next in graph[node].iter() {
//                 // nextの周りを探索
//                 dfs2(next, Some(node), graph, dp, m);
//             }
//         }
//     }
// }

// fn mod_inv(x: isize, m: isize) -> isize {
//     let mut a = vec![x, m];
//     let mut b = vec![1_isize, 0_isize];
//     while a[0] > 0 && a[1] > 0 {
//         let t = a[0] / a[1];
//         a[0] -= t * a[1];
//         a.reverse();
//         b[0] -= t * b[1];
//         b.reverse();
//     }
//     b[0] %= m;
//     if b[0] < 0 {
//         b[0] += m;
//     }
//     return b[0];
// }

fn mod_inv(x: isize, m: isize) -> isize {
    let mut a = vec![x, m];
    let mut b = vec![1, 0];
    while a[1] != 0 {
        let t = a[0] / a[1];
        a[0] -= t * a[1];
        a.swap(0, 1);
        b[0] -= t * b[1];
        b.swap(0, 1);
    }
    b[0] %= m;
    if b[0] < 0 {
        b[0] += m;
    }
    b[0]
}
