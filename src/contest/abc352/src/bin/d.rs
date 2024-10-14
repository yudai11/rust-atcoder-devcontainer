use proconio::{input, marker::Usize1};
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
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, k: usize,
        p: [Usize1;n]
    }

    let mut k_conti_set = BTreeSet::new();
    let mut pos_num = vec![0; n];
    let mut ans = n - 1;
    if n == k {
        println!("{}", n - 1);
        return;
    }
    for i in 0..n {
        pos_num[p[i]] = i;
    }
    for i in 0..n {
        if i < k {
            k_conti_set.insert(pos_num[i]);
            if k_conti_set.len() == k {
                ans = ans.min(k_conti_set.last().unwrap() - k_conti_set.first().unwrap());
            }
        } else {
            k_conti_set.remove(&pos_num[i - k]);
            k_conti_set.insert(pos_num[i]);
            ans = ans.min(k_conti_set.last().unwrap() - k_conti_set.first().unwrap());
        }
    }
    println!("{ans}");
}
