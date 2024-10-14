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
        a: [usize;n]
    }

    let xor_all = a.iter().fold(0 as usize, |res, &x| res ^ x);
    // let mut ans = vec![0 as usize; n];
    for i in 0..n {
        let res = xor_all ^ a[i];
        print!("{res} ");
    }
}
