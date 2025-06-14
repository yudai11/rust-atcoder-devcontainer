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
        _s: Chars
    }

    let n = _s.len();

    let mut s = vec![];

    for &si in _s.iter() {
        s.push(si as usize - '0' as usize);
    }

    let mut ans = 1_usize;
    let mut t = s[0];

    for i in 1..n {
        ans += (t + 10 - s[i]) % 10 + 1;
        t = s[i];
    }

    ans += s[n - 1];

    println!("{}", ans);
}
