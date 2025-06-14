use itertools::Itertools;
use proconio::input;
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
        x: [usize; q]
    }

    let mut boxes = vec![0_usize; n];
    let mut loc = vec![0_usize; q];

    for i in 0..q {
        if x[i] == 0 {
            let mut min_val = 100_usize;
            for &bi in boxes.iter() {
                min_val = min_val.min(bi);
            }
            for j in 0..n {
                if boxes[j] == min_val {
                    loc[i] = j + 1;
                    boxes[j] += 1;
                    break;
                }
            }
        } else {
            loc[i] = x[i];
            boxes[x[i] - 1] += 1;
        }
    }

    println!("{}", loc.iter().join(" "));
}
