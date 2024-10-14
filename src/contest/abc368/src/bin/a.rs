use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use petgraph::unionfind::UnionFind;

fn main() {
    input! {
         n: usize,
         k: usize,
         a: [usize; n]
    }

    for i in n - k..n {
        print!("{} ", a[i]);
    }
    for i in 0..n - k {
        print!("{} ", a[i]);
    }
}
