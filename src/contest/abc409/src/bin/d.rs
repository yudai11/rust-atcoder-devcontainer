use itertools::Itertools;
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
        t: usize
    }

    let mut ans = vec![];

    for _i in 0..t {
        input! {
            n: usize,
            s: Chars
        }

        let mut l = 0_usize;
        while l + 1 < n && (s[l] as usize) <= (s[l + 1] as usize) {
            l += 1;
        }

        let mut r = l;
        while r + 1 < n && (s[l] as usize) >= (s[r + 1] as usize) {
            r += 1;
        }

        let mut t = s.clone();
        t[r] = s[l];
        for i in l..r {
            t[i] = s[i + 1];
        }

        ans.push(t.iter().join(""));
    }

    println!("{}", ans.iter().join("\n"));
}
