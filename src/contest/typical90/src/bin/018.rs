use itertools::Itertools;
use proconio::input;
use std::f64::consts::PI;
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
        t: f64,
        l: f64, x: f64, y: f64,
        q: usize,
        e: [f64;q]
    }

    let mut ans = vec![];

    for &ei in e.iter() {
        let ei = ei % t;
        let angle = ei * 2.0 / t * PI;
        let loc_e8 = (0.0, -l / 2.0 * angle.sin(), l / 2.0 - l / 2.0 * angle.cos());
        let hori_gap = (x * x + (loc_e8.1 - y) * (loc_e8.1 - y)).sqrt();
        let verti_gap = loc_e8.2;
        ans.push(verti_gap.atan2(hori_gap) / PI * 180.0);
    }

    println!("{}", ans.iter().join("\n"));
}
