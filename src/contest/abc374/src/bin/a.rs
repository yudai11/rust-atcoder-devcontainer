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

    let n = s.len();

    if s[n - 3] == 's' && s[n - 2] == 'a' && s[n - 1] == 'n' {
        println!("Yes");
    } else {
        println!("No");
    }
}
