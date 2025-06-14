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
        n: usize,
        a: [Usize1; n]
    }

    let mut change_points = vec![];
    change_points.push(0);

    for i in 1..n - 1 {
        if (a[i - 1] < a[i] && a[i] > a[i + 1]) || (a[i - 1] > a[i] && a[i] < a[i + 1]) {
            change_points.push(i);
        }
    }

    change_points.push(n - 1);

    let m = change_points.len();
    let mut ans = 0_usize;

    for i in 0..m.max(3) - 3 {
        if a[change_points[i]] > a[change_points[i + 1]] {
            continue;
        }
        ans += (change_points[i + 1] - change_points[i])
            * (change_points[i + 3] - change_points[i + 2])
    }

    println!("{}", ans);
}
