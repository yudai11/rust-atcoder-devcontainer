use std::{collections::HashSet, fmt::format};

use itertools::Itertools;
use proconio::input;
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
        n: usize,
        s: [String; n]
    }

    let mut seen = HashSet::new();
    let mut ans = 0_usize;

    for c in (0..n).combinations(2) {
        let x = format!("{}{}", s[c[0]], s[c[1]]);
        let y = format!("{}{}", s[c[1]], s[c[0]]);

        if !seen.contains(&x) {
            ans += 1;
            seen.insert(x);
        }
        if !seen.contains(&y) {
            ans += 1;
            seen.insert(y);
        }
    }

    println!("{}", ans);
}
