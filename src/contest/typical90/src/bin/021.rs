use ac_library::SccGraph;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1,Usize1); m]
    }

    let mut ans = 0_usize;

    // SCC (強連結成分分解グラフ) ac-library
    let mut graph = SccGraph::new(n);

    for &(u, v) in edges.iter() {
        graph.add_edge(u, v);
    }

    let decomp_list = graph.scc();

    for x in decomp_list.iter() {
        let k = x.len();
        ans += k * (k - 1) / 2;
    }

    println!("{}", ans);
}
