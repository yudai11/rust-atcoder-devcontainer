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
        n: usize, m: usize,
        a: [usize; n]
    }

    let mut ans = 0_usize;
}

use ac_library::{Additive, LazySegtree, MapMonoid, Monoid};
struct S {}
impl MapMonoid for S {
    type M = Additive<usize>; // 区間クエリは各区間の合計を返す
    type F = usize; // 更新に使う値の型は usize

    // 更新がないときの恒等写像（ここでは「何も更新しない」or 「初期値」を usize::MAX とする）
    fn identity_map() -> Self::F {
        0
    }

    // mapping: 遅延更新の適用時、更新値 f と現在の値 x の合計を返す
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        *f + x
    }

    // composition: 2 つの更新 f と g を合成する際、2 つの合計を用いる
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        *f + g
    }
}
