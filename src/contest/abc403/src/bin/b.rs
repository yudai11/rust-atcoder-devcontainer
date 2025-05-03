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
        t: Chars,
        u: Chars,
    }

    let n = t.len();
    let m = u.len();

    // let mut cnt = 0_usize;

    for i in 0..=n - m {
        if t[i] != u[0] && t[i] != '?' {
            continue;
        }

        let mut feasi = true;
        for j in 1..m {
            if t[i + j] == '?' || t[i + j] == u[j] {
                continue;
            } else {
                feasi = false;
                // break;
            }
        }

        if feasi {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
