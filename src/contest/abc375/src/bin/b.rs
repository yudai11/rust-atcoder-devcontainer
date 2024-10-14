use proconio::input;
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
// use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        xy: [(f64,f64);n]
    }

    let mut ans = 0.0;
    let mut cur_pt = (0.0, 0.0);

    for i in 0..n {
        ans += dist(cur_pt, xy[i]);
        cur_pt = xy[i];
    }

    ans += dist((0.0, 0.0), cur_pt);

    println!("{ans}");
}

fn dist(x: (f64, f64), y: (f64, f64)) -> f64 {
    return ((x.0 - y.0) * (x.0 - y.0) + (x.1 - y.1) * (x.1 - y.1)).sqrt();
}
