use proconio::{input, marker::Chars};
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
        s: [char;3]
    }

    let mut ages = [0; 3];
    let alp = ['A', 'B', 'C'];
    if s[0] == '<' {
        ages[1] += 1;
    } else {
        ages[0] += 1;
    }

    if s[1] == '<' {
        ages[2] += 1;
    } else {
        ages[0] += 1;
    }

    if s[2] == '<' {
        ages[2] += 1;
    } else {
        ages[1] += 1;
    }

    for i in 0..3 {
        if ages[i] == 1 {
            println!("{}", alp[i]);
        }
    }
}
