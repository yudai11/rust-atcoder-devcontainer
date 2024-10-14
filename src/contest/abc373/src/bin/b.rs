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

    let mut loc_list: [isize; 26] = [0; 26];

    for i in 0..26 {
        let v = s[i];
        loc_list[v as usize - 'A' as usize] = i as isize;
    }

    let mut ans = 0;
    for i in 0..25 {
        ans += (loc_list[i + 1] - loc_list[i]).abs();
    }

    println!("{ans}");
}
