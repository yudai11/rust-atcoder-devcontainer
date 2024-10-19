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
        n: usize, c: usize,
        t: [usize;n]
    }

    let mut ans = 1;
    let mut last_time = t[0];
    for i in 1..n {
        if t[i] - last_time >= c {
            ans += 1;
            last_time = t[i];
        }
    }

    println!("{ans}");
}
