use proconio::input;
// use proconio::marker::Chars;
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
        n: usize,
        d: [usize; n]
    }

    let mut ans = 0_usize;

    for i in 1..=n {
        let zoro = i % 10;
        let mut cp_i = i / 10;
        let mut feasi = zoro != 0;
        while cp_i > 0 {
            if cp_i % 10 != zoro {
                feasi = false;
                break;
            }
            cp_i /= 10;
        }
        if !feasi {
            continue;
        }
        for j in 1..=d[i - 1] {
            let mut cp_j = j;
            let mut feasi = true;
            while cp_j > 0 {
                if cp_j % 10 != zoro {
                    feasi = false;
                    break;
                }
                cp_j /= 10;
            }
            if feasi {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
