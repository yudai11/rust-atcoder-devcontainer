use proconio::{input, marker::Chars};
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
        s: Chars
    }

    let l = s.len();
    let mut i = 0_usize;
    let mut ans = 0_usize;

    // 0なら次はi
    let mut state = 0_usize;

    while i < l {
        match state {
            0 => {
                if s[i] == 'i' {
                    i += 1;
                } else {
                    ans += 1;
                }
                state = 1;
            }
            1 => {
                if s[i] == 'o' {
                    i += 1;
                } else {
                    ans += 1;
                }
                state = 0;
            }
            _ => unreachable!(),
        }
    }

    if state == 1 {
        ans += 1;
    }

    println!("{}", ans);
}
