use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
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
        k: usize, g: isize, m: isize,
    }

    let mut g_w = 0_isize;
    let mut m_w = 0_isize;
    for i in 0..k {
        if g_w == g {
            g_w = 0;
        } else if m_w == 0 {
            m_w = m;
        } else if m_w >= g - g_w {
            m_w -= g - g_w;
            g_w = g;
        } else {
            g_w += m_w;
            m_w = 0;
        }
    }

    println!("{} {}", g_w, m_w);
}
