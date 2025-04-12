use proconio::{input, marker::Usize1};

// 全方位木DP
fn main() {
    input! {
        n: usize,
        edges: [(Usize1,Usize1);n-1]
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        graph[u].push(v);
        graph[v].push(u);
    }

    // 最遠点との距離 /　木の直径
    let mut dp = vec![0_usize; n];

    dfs1(0, None, &graph, &mut dp);
    dfs2(0, None, &graph, &mut dp, 0);

    let ans = dp.iter().fold(0_usize, |res, &x| res.max(x)) + 1;
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
