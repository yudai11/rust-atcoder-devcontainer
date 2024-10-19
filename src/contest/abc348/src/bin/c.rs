use std::collections::{HashMap, HashSet};

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
        ac: [(usize,usize);n]
    }

    let mut minimum_taste: HashMap<usize, usize> = HashMap::new();
    for &(a, c) in ac.iter() {
        if minimum_taste.contains_key(&c) && minimum_taste[&c] > a {
            minimum_taste.remove(&c);
            minimum_taste.insert(c, a);
        } else if !minimum_taste.contains_key(&c) {
            minimum_taste.insert(c, a);
        }
    }

    let ans = minimum_taste.iter().fold(0, |ans, (&_k, &v)| ans.max(v));
    println!("{ans}");
}
