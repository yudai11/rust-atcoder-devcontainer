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
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;

fn main() {
    input! {
        s: Chars,
        T: Chars
    }
    let mut S = vec!['A'; s.len()];
    for i in 0..s.len() {
        S[i] = s[i].to_ascii_uppercase();
    }

    if T[2] == 'X' {
        let mut progress = 0;
        for &v in S.iter() {
            if progress == 0 && v == T[0] {
                progress += 1;
            } else if progress == 1 && v == T[1] {
                println!("Yes");
                return;
            }
        }
    } else {
        let mut progress = 0;
        for &v in S.iter() {
            if progress == 0 && v == T[0] {
                progress += 1;
            } else if progress == 1 && v == T[1] {
                progress += 1;
            } else if progress == 2 && v == T[2] {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
