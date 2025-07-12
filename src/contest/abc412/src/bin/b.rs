use std::collections::HashSet;

use proconio::{input, marker::Chars};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;
// use superslice::Ext; // for use of lowerbound upperbound method of vetor

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let mut t_letter_set = HashSet::new();
    for &ti in t.iter() {
        t_letter_set.insert(ti);
    }

    let n = s.len();

    for i in 0..n - 1 {
        if s[i + 1].is_ascii_uppercase() && !t_letter_set.contains(&s[i]) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
