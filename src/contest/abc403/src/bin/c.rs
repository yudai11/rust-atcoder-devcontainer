use std::collections::HashSet;

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
        n: usize, m: usize, q: usize,
    }

    let mut can_seen = vec![HashSet::new(); n];
    let mut can_see_all = vec![false; n];

    let mut ans = vec![];

    for _i in 0..q {
        input! {
            t: u8
        }

        match t {
            1 => {
                input! {
                    x: Usize1, y: Usize1
                }
                can_seen[x].insert(y);
            }
            2 => {
                input! {
                    x: Usize1
                }

                can_see_all[x] = true;
            }
            3 => {
                input! {
                    x: Usize1, y: Usize1
                }

                if can_see_all[x] || can_seen[x].contains(&y) {
                    ans.push("Yes");
                } else {
                    ans.push("No");
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans.iter().join("\n"));
}
