use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        w: usize, n: usize,
        lr: [(Usize1,Usize1); n]
    }

    let mut ans = vec![];

    let mut segtree = LazySegtree::<S>::new(w);
    for &(l, r) in lr.iter() {
        let new_val = segtree.prod(l..=r) + 1;
        segtree.apply_range(l..=r, new_val);
        ans.push(new_val);
    }

    println!("{}", ans.iter().join("\n"));
}

// Lazy Segment tree (MAX) 遅延セグ木(max)
use ac_library::{LazySegtree, MapMonoid, Max, Monoid};
struct S {}

impl MapMonoid for S {
    type M = Max<usize>; // 区間クエリは各区間の最大値を返す
    type F = usize; // 更新に使う値の型は i32

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
