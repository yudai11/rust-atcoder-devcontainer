use proconio::input;
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
        a: usize,
        b: usize,
    }
    if a == b {
        println!("1");
    } else if (a + b) % 2 != 0 {
        println!("2");
    } else {
        println!("3");
    }
}
