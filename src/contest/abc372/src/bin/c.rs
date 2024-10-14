use proconio::{
    input,
    marker::{Chars, Usize1},
};
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
        n: usize, q: usize,
        mut s: Chars,
        queries: [(Usize1, char); q]
    }

    let mut num_abc = 0;
    for i in 0..(n - 2) {
        if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
            num_abc += 1;
        }
    }

    for i in 0..q {
        let (x, c) = queries[i];
        if c == s[x] {
            println!("{num_abc}");
            continue;
        }
        if x < n - 2 && s[x] == 'A' && s[x + 1] == 'B' && s[x + 2] == 'C' {
            num_abc -= 1;
        } else if x < n - 1 && x > 0 && s[x - 1] == 'A' && s[x] == 'B' && s[x + 1] == 'C' {
            num_abc -= 1;
        } else if x < n && x > 1 && s[x - 2] == 'A' && s[x - 1] == 'B' && s[x] == 'C' {
            num_abc -= 1;
        }

        if x < n - 2 && c == 'A' && s[x + 1] == 'B' && s[x + 2] == 'C' {
            num_abc += 1;
        } else if x < n - 1 && x > 0 && c == 'B' && s[x - 1] == 'A' && s[x + 1] == 'C' {
            num_abc += 1;
        } else if x < n && x > 1 && c == 'C' && s[x - 2] == 'A' && s[x - 1] == 'B' {
            num_abc += 1;
        }

        s[x] = c;
        println!("{num_abc}");
    }
}
