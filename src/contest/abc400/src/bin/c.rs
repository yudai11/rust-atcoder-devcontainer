use num_integer::Roots;
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
        n: u128
    }

    let mut pows_2 = vec![];
    let mut pow2 = 2_u128;
    while pow2 <= n {
        pows_2.push(pow2);
        pow2 *= 2;
    }

    let mut ans = 0_usize;
    for &x in pows_2.iter() {
        let y = ((n / x) as usize).sqrt();
        ans += y / 2;
        if y % 2 != 0 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
