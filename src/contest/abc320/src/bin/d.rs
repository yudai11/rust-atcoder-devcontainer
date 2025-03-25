use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize,
        mut abxy: [(Usize1,Usize1,isize,isize); m]
    }

    let mut graph = vec![vec![]; n];
    for &(a, b, x, y) in abxy.iter() {
        graph[a].push((b, x, y));
        graph[b].push((a, -x, -y));
    }

    let mut locs = vec![(0_isize, 0_isize); n];
    let mut uf = Dsu::new(n);
    let mut seen = vec![false; n];

    dfs(0, &graph, &mut seen, &mut uf, &mut locs);

    let mut ans = vec![];

    for i in 0..n {
        if uf.same(0, i) {
            let res = format!("{} {}", (locs[i].0).to_string(), (locs[i].1).to_string());
            ans.push(res);
        } else {
            ans.push(String::from("undecidable"));
        }
    }

    println!("{}", ans.iter().join("\n"));
}

fn dfs(
    node: usize,
    graph: &Vec<Vec<(usize, isize, isize)>>,
    seen: &mut Vec<bool>,
    uf: &mut Dsu,
    locs: &mut Vec<(isize, isize)>,
) {
    seen[node] = true;
    for &next in graph[node].iter() {
        if !seen[next.0] {
            uf.merge(node, next.0);
            locs[next.0].0 = locs[node].0 + next.1;
            locs[next.0].1 = locs[node].1 + next.2;
            dfs(next.0, graph, seen, uf, locs)
        }
    }
}
