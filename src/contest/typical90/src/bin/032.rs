use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
    }

    let mut graph = vec![vec![]; n];

    {
        input! {
            xy: [(Usize1,Usize1); m]
        }
        let mut ng_list = vec![vec![]; n];
        for &(x, y) in xy.iter() {
            ng_list[x].push(y);
            ng_list[y].push(x);
        }

        for i in 0..n {
            for j in i + 1..n {
                if !ng_list[i].contains(&j) {
                    graph[i].push(j);
                    graph[j].push(i);
                }
            }
        }
    }

    let mut ans = 1000_000_usize;
    let mut seen = vec![false; n];
    for i in 0..n {
        dfs(i, &graph, &a, &mut seen, &mut ans, 1, n, a[i][0]);
    }

    if ans > 1000_00 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn dfs(
    node: usize,
    graph: &Vec<Vec<usize>>,
    a: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    ans: &mut usize,
    depth: usize,
    n: usize,
    cost: usize,
) {
    if depth == n {
        *ans = (*ans).min(cost);
        return;
    }
    seen[node] = true;
    for &next in graph[node].iter() {
        if !seen[next] {
            dfs(
                next,
                graph,
                a,
                seen,
                ans,
                depth + 1,
                n,
                cost + a[next][depth],
            );
        }
    }
    seen[node] = false;
}
