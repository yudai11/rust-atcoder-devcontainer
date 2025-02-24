use proconio::input;
use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
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
        _n: usize, m: usize,
        s: Chars
    }

    let mut logo_t = 0_usize;

    let mut worn_logo_t = 0_usize;
    let mut worn_plain_t = 0_usize;

    for &si in s.iter() {
        match si {
            '0' => {
                logo_t = logo_t.max(worn_logo_t);
                worn_logo_t = 0;
                worn_plain_t = 0;
            }
            '1' => {
                if worn_plain_t < m {
                    worn_plain_t += 1;
                } else {
                    worn_logo_t += 1;
                }
            }
            '2' => {
                worn_logo_t += 1;
            }
            _ => unreachable!(),
        }
    }

    logo_t = logo_t.max(worn_logo_t);

    println!("{}", logo_t);
}
