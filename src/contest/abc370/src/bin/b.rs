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
        n: usize,
        a: [Usize1; (n+1) * n / 2]
    }

    let mut k = 0;
    let mut list = vec![vec![]; n];
    for i in 0..n {
        for _j in 0..=i {
            list[i].push(a[k]);
            k += 1;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        ans = compose(ans, i, &list);
    }

    println!("{}", ans + 1);
}

fn compose(i: usize, j: usize, list: &Vec<Vec<usize>>) -> usize {
    let greater = i.max(j);
    let lesser = i.min(j);
    list[greater][lesser]
}
