use proconio::{input, marker::Usize1};
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
        n: usize, d: usize, rr: usize,
        h: [Usize1; n]
    }

    let mut p = vec![0_usize; n];
    for (i, &hi) in h.iter().enumerate() {
        p[hi] = i;
    }

    let mut segtree = LazySegtree::<S>::new(n);
    for i in 0..n {
        segtree.set(i, -1000_000_000_000_000_000 as isize);
    }
    let mut dp = vec![0_isize; n];

    for i in 0..n {
        if d <= i {
            segtree.set(p[i - d], dp[p[i - d]]);
        }
        let (l, r) = (p[i].saturating_sub(rr), (p[i] + rr).min(n - 1));
        dp[p[i]] = dp[p[i]].max(segtree.prod(l..=r) + 1);
    }

    let ans = dp.iter().fold(0_isize, |max_val, &x| max_val.max(x));
    println!("{}", ans - 1);
}

use ac_library::{LazySegtree, MapMonoid, Max, Monoid};
struct S {}

impl MapMonoid for S {
    type M = Max<isize>; // 区間クエリは各区間の最大値を返す
    type F = isize; // 更新に使う値の型は usize

    // 更新がないときの恒等写像（ここでは「何も更新しない」を 0 とする）
    fn identity_map() -> Self::F {
        0
    }

    // mapping: 遅延更新の適用時、更新値 f と現在の値 x のうち大きいほうを返す
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        *f.max(x)
    }

    // composition: 2 つの更新 f と g を合成する際、2 つのうち大きいほうを用いる
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        *f.max(g)
    }
}
