use num_integer::Roots;
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
        d: usize
    }

    let mut ans = d;

    let l = d.sqrt() + 1;
    let mut x = 0_usize;
    let mut y: usize = l;

    while x <= l {
        if y * y + x * x > d && y > 0 {
            ans = ans.min(y * y + x * x - d);
            y -= 1;
        } else if y * y + x * x < d {
            ans = ans.min(d - y * y - x * x);
            x += 1;
        } else {
            ans = ans.min(y * y + x * x - d);
            break;
        }
    }

    println!("{}", ans);
}
