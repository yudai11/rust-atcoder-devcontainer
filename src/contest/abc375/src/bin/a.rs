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
        n:usize,
        s: Chars
    }

    let mut ans = 0;

    for i in 1..(n - 1) {
        if s[i] == '.' && s[i - 1] == '#' && s[i + 1] == '#' {
            ans += 1;
        }
    }

    println!("{ans}");
}
