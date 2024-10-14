use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
use std::collections::HashSet;
// use std::collections::VecDeque;
use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1,Usize1);m]
    }

    let mut uf = UnionFind::new(n);
    for i in 0..m {
        let (x, y) = edges[i];
        uf.union(x, y);
    }

    let mut root_set = HashSet::new();
    let mut num_comp = vec![0; n];
    for i in 0..n {
        let x = uf.find(i);
        root_set.insert(x);
        num_comp[x] += 1;
    }

    let mut ans = 0;
    for &x in root_set.iter() {
        let n = num_comp[x];
        // println!("n: {}", n);
        ans += n * (n - 1) / 2;
    }

    println!("{}", ans - m);
}
