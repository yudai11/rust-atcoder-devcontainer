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
        n: usize, m: usize,
        mut b: [isize; n],
        mut w: [isize; m]
    }

    b.sort();
    b.reverse();
    w.sort();
    w.reverse();

    let l = n.min(m);
    let mut ans = 0_isize;
    let mut i = 0_usize;

    while i < l {
        if b[i] + w[i] <= 0 && b[i] <= 0 {
            break;
        } else if b[i] + w[i] <= 0 {
            ans += b[i];
        } else {
            ans += b[i];
            if w[i] > 0 {
                ans += w[i];
            }
        }
        i += 1;
    }

    while i < n {
        if b[i] <= 0 {
            break;
        }
        ans += b[i];
        i += 1;
    }

    println!("{}", ans);
}
