use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;
// use superslice::Ext; // for use of lowerbound upperbound method of vetor

fn main() {
    input! {
        a: [Usize1; 7]
    }
    let mut cnt = vec![0_usize; 13];
    for &ai in a.iter() {
        cnt[ai] += 1;
    }

    let mut num_2pair = 0_usize;
    let mut num_3pair = 0_usize;

    for &ci in cnt.iter() {
        if ci >= 2 {
            num_2pair += 1;
        }
        if ci >= 3 {
            num_3pair += 1;
        }
    }

    if num_2pair >= 2 && num_3pair >= 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
