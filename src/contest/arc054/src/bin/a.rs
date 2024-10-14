use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;

fn main() {
    input! {
        l: f64, x: f64, y: f64, s: f64, d: f64
    }

    if s == d {
        println!("0");
    } else if s < d {
        let mut ans = (d - s) / (x + y);
        if y > x {
            ans = ans.min((l + s - d) / (y - x));
        }
        println!("{ans}")
    } else if s > d {
        let mut ans = (l + d - s) / (x + y);
        if y > x {
            ans = ans.min((s - d) / (y - x));
        }
        println!("{ans}")
    }
}
