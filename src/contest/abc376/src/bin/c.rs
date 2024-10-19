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
        mut a : [usize;n],
        mut b : [usize;n-1]
    }

    a.sort();
    a.reverse();
    b.push(0);
    b.sort();
    b.reverse();

    let mut not_in = vec![];
    let mut i = 0;
    let mut j = 0;
    while i < n && j < n {
        let ai = a[i];
        let bj = b[j];
        if ai <= bj {
            i += 1;
            j += 1;
        } else {
            not_in.push(ai);
            i += 1;
        }
    }

    for k in i..n {
        not_in.push(a[k]);
    }

    if not_in.len() > 1 {
        println!("-1");
    } else {
        println!("{}", not_in[0]);
    }
}
