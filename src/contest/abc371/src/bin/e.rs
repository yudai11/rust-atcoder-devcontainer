// use im_rc::HashSet;
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
// use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        a:[Usize1;n]
    }

    let mut ans = 0;
    let mut loc = vec![vec![]; n];
    for (i, &v) in a.iter().enumerate() {
        loc[v].push(i);
    }

    for i in 0..n {
        if loc[i].len() == 0 {
            continue;
        }
        ans += f(n);
        ans -= f(loc[i][0]);
        for v in loc[i].windows(2) {
            let (l, r) = (v[0], v[1]);
            ans -= f(r - l - 1);
        }
        ans -= f(n - loc[i].last().unwrap() - 1);
    }

    println!("{ans}");
}

fn f(x: usize) -> usize {
    if x == 0 {
        return 0;
    } else {
        return x * (x + 1) / 2;
    }
}
