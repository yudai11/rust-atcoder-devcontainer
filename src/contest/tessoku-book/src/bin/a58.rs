use ac_library::{LazySegtree, MapMonoid, Max, Monoid};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, q: usize
    }

    let mut segtree = LazySegtree::<S>::new(n + 1);
    let mut ans = vec![];

    for _i in 0..q {
        input! {
            t: usize
        }
        match t {
            1 => {
                input! {
                    pos: Usize1, x: usize
                }
                segtree.set(pos, x);
            }
            2 => {
                input! {
                    l: Usize1, r: Usize1
                }
                ans.push(segtree.prod(l..r));
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans.iter().join("\n"));
}

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
