use itertools::Itertools;
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
        n: usize, q: usize,
    }

    let mut a = vec![0_usize; n];
    for i in 0..n {
        a[i] = i + 1;
    }

    let mut top_loc = 0_usize;
    let mut ans = vec![];

    for _i in 0..q {
        input! {
            t: u8
        }

        match t {
            1 => {
                input! {
                    p: Usize1, x: usize
                }
                a[(p + top_loc) % n] = x;
            }
            2 => {
                input! {
                    p: Usize1
                }

                ans.push(a[(p + top_loc) % n]);
            }
            3 => {
                input! {
                    k: usize
                }

                top_loc += k;
                top_loc %= n;
            }
            _ => unreachable!(),
        }
    }

    if ans.len() > 0 {
        println!("{}", ans.iter().join("\n"));
    }
}
