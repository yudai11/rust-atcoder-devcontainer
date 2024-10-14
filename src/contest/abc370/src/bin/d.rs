use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use petgraph::unionfind::UnionFind;
use std::collections::BTreeSet;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// use std::collections::BTreeSet;

fn main() {
    input! {
        (h, w, q): (usize,usize,usize),
        query: [(Usize1,Usize1);q]
    }

    let mut verti_set: Vec<BTreeSet<_>> = vec![(0..h).collect(); w];
    let mut hori_set: Vec<BTreeSet<_>> = vec![(0..w).collect(); h];
    // for i in 0..w {
    //     for j in 0..h {
    //         verti_set[i].insert(j);
    //         hori_set[j].insert(i);
    //     }
    // }

    for &(r, c) in &query {
        if verti_set[c].contains(&r) {
            verti_set[c].remove(&r);
            hori_set[r].remove(&c);
            continue;
        }
        if let Some(&j) = verti_set[c].range(..r).last() {
            verti_set[c].remove(&j);
            hori_set[j].remove(&c);
        }
        if let Some(&j) = verti_set[c].range(r..).next() {
            verti_set[c].remove(&j);
            hori_set[j].remove(&c);
        }
        if let Some(&i) = hori_set[r].range(..c).last() {
            verti_set[i].remove(&r);
            hori_set[r].remove(&i);
        }
        if let Some(&i) = hori_set[r].range(c..).next() {
            verti_set[i].remove(&r);
            hori_set[r].remove(&i);
        }
    }

    println!("{}", verti_set.iter().map(|x| x.len()).sum::<usize>());
}
