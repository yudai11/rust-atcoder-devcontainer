use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;


fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        rc: [(Usize1,Usize1);n]
    }

    let mut map = vec![vec![0;w]; h];
    for i in 0..n {
        map[rc[i].0][rc[i].1] += 1;
    }

}
