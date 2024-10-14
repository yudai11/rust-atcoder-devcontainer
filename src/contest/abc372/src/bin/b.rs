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
        m: usize
    }

    let mut cp_m = m;
    let mut a = vec![];
    let mut j = 0;
    while cp_m > 0 {
        let k = cp_m % 3;
        for _i in 0..k {
            a.push(j);
        }
        j += 1;
        cp_m /= 3;
    }

    let n = a.len();
    println!("{n}");
    for &x in &a {
        print!("{x} ");
    }
}
