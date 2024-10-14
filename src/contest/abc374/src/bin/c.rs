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
        n: usize,
        k: [usize; n]
    }

    let mut ans = k.iter().fold(0, |sum, &x| sum + x);

    for i in 0..(1 << n) {
        let mut a = 0;
        let mut b = 0;

        for j in 0..n {
            if (i >> j) & 1 == 1 {
                a += k[j];
            } else {
                b += k[j];
            }
        }

        ans = ans.min(a.max(b));
    }

    println!("{ans}");
}
