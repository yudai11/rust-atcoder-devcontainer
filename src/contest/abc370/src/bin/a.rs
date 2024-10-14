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
        (l, r): (usize,usize)
    }

    if l == 1 && r == 0 {
        println!("Yes");
    } else if l == 0 && r == 1 {
        println! {"No"};
    } else {
        println!("Invalid");
    }
}
