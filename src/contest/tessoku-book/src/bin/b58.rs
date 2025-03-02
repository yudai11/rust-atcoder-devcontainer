use proconio::input;
use superslice::Ext; // for use of lowerbound upperbound method of vetor

fn main() {
    input! {
        n: usize, l: usize, r: usize,
        x: [usize; n]
    }

    // i番目の足場にたどり着くまでの最小ターン数
    let mut segtree = LazySegtree::<S>::new(n);
    segtree.set(0, 0);

    for (i, &loc) in x.iter().enumerate().skip(1) {
        let (ll, rr) = (
            x.lower_bound(&(loc.max(r) - r)),
            x.upper_bound(&(loc.max(l) - l)),
        );
        let min_val = segtree.prod(ll..rr);
        if min_val < 2 * n {
            segtree.set(i, min_val + 1);
        }
    }

    println!("{}", segtree.get(n - 1));
}

use ac_library::{LazySegtree, MapMonoid, Min, Monoid, Sum};
struct S {}

impl MapMonoid for S {
    type M = Sum<usize>; // 区間クエリは各区間の最大値を返す
    type F = usize; // 更新に使う値の型は usize

    // 更新がないときの恒等写像（ここでは「何も更新しない」or 「初期値」を usize::MAX とする）
    fn identity_map() -> Self::F {
        0
    }

    // mapping: 遅延更新の適用時、更新値 f と現在の値 x のうち小さいほうを返す
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        *f
    }

    // composition: 2 つの更新 f と g を合成する際、2 つのうち小さいほうを用いる
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        *f + g
    }
}
