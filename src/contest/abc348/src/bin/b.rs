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
        xy: [(isize,isize);n]
    }

    for i in 0..n {
        let (xi, yi) = xy[i];
        let mut ans = 0;
        let mut max_dist = 0;
        for j in 0..n {
            if j == i {
                continue;
            }
            let (xj, yj) = xy[j];
            if max_dist < (xi - xj) * (xi - xj) + (yi - yj) * (yi - yj) {
                max_dist = (xi - xj) * (xi - xj) + (yi - yj) * (yi - yj);
                ans = j + 1;
            }
        }

        println!("{ans}");
    }
}
