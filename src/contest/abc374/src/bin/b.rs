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
        s: Chars,
        t: Chars
    }

    let n = s.len().min(t.len());
    let mut res = n + 1;

    for i in 0..n {
        if s[i] != t[i] {
            res = i + 1;
            break;
        }

        if i == n - 1 && s.len() == t.len() {
            println!("0");
            return;
        }
    }

    println!("{res}");
}
