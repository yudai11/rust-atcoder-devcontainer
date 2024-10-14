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
        s: [String; n]
    }

    let mut ans: usize = 1;
    for (i, si) in s.iter().enumerate() {
        if si == "OR" {
            ans += 1 << (i + 1);
        }
    }

    println!("{ans}");
}
