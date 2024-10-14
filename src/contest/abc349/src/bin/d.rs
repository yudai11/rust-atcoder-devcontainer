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
        l: usize, r: usize
    }

    let mut m = 0;
    let mut division_list: Vec<(usize, usize)> = vec![];
    let mut left = l;
    while left < r {
        let mut range = 1;
        let mut cp_left = left;
        while cp_left % 2 == 0 && left + 2 * range <= r {
            cp_left /= 2;
            range *= 2;
        }
        m += 1;
        division_list.push((left, left + range));
        left = left + range;
    }
    println!("{m}");
    for &(l, r) in division_list.iter() {
        println!("{l} {r}");
    }
}
