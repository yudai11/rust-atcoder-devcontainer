use ac_library::{LazySegtree, MapMonoid, Max, Monoid};
use proconio::input;

fn main() {
    input! {
        w: usize, n: usize,
        lrv: [(usize,usize, isize); n]
    }

    // オンラインで香辛料をちょうど j 使ったときの最大価値を更新する．
    let mut dp = LazySegtree::<S>::new(w + 1);
    let mut temp_list = vec![-1_isize; w + 1];

    dp.set(0, 0);

    for &(l, r, v) in lrv.iter() {
        for j in l..=w {
            let max_val = dp.prod(j.max(r) - r..=j - l);
            if max_val != -1 {
                temp_list[j] = temp_list[j].max(max_val + v);
            }
        }
        for j in l..=w {
            dp.set(j, temp_list[j]);
        }
    }

    println!("{}", temp_list[w]);
}

// 初期値 -1 をもつ Max Lazy Segtree
struct S {}
impl MapMonoid for S {
    type M = Max<isize>; // 区間クエリは各区間の最大値を返す
    type F = isize; // 更新に使う値の型は usize

    // 更新がないときの恒等写像（ここでは「何も更新しない」を 0 とする）
    fn identity_map() -> Self::F {
        -1
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
