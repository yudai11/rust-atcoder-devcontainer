use proconio::{input, marker::Chars};
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
        s: Chars
    }

    let mut ans: usize = 0;
    let n = s.len();
    let mut cumsum: Vec<Vec<usize>> = vec![vec![0; n]; 26];

    for i in 0..n {
        let v = s[i] as usize - 'A' as usize;
        cumsum[v][i] += 1;
    }

    for i in 0..26 {
        for j in 0..(n - 1) {
            cumsum[i][j + 1] += cumsum[i][j];
        }
    }

    for j in 1..n - 1 {
        for k in 0..26 {
            let left_k = cumsum[k][j - 1];
            let right_k = cumsum[k][n - 1] - cumsum[k][j];
            ans += left_k * right_k;
        }
    }

    println!("{ans}");
}
