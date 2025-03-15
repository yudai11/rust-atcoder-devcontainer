use proconio::{input, marker::Usize1};

fn main() {
    input! {
     n: usize,
     edges: [(Usize1,Usize1); n-1]
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut dp = vec![1_usize; n];
    let mut ans = 0_usize;

    dfs(0, None, &graph, &mut dp, n, &mut ans);

    println!("{}", ans);
}

// 木DP
fn dfs(
    node: usize,
    parent: Option<usize>,
    graph: &Vec<Vec<usize>>,
    dp: &mut Vec<usize>,
    n: usize,
    ans: &mut usize,
) {
    match parent {
        Some(p) => {
            for &next in graph[node].iter() {
                if next != p {
                    dfs(next, Some(node), graph, dp, n, ans);
                    dp[node] += dp[next];
                    // その辺を通るような点の組の数を加える(主客転倒)
                    *ans += dp[next] * (n - dp[next]);
                }
            }
        }
        None => {
            for &next in graph[node].iter() {
                dfs(next, Some(node), graph, dp, n, ans);
                dp[node] += dp[next];
                *ans += dp[next] * (n - dp[next]);
            }
        }
    }
}
