use itertools::Itertools;
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
        n: usize
    }

    if n % 2 != 0 {
        return;
    }

    for i in 0..(1 << n) as usize {
        let mut p = vec![];
        let mut stack = vec![];
        for j in (0..n).rev() {
            let x = (i >> j) % 2;
            p.push(x);
            if stack.len() == 0 {
                stack.push(x);
            } else if let Some(y) = stack.pop() {
                match y {
                    0 => {
                        if x != 1 {
                            stack.push(y);
                            stack.push(x);
                        }
                    }
                    _ => {
                        stack.push(y);
                        stack.push(x);
                    }
                }
            }
        }

        if stack.len() == 0 {
            let mut res = vec![];
            for &pi in p.iter() {
                match pi {
                    0 => res.push('('),
                    1 => res.push(')'),
                    _ => unreachable!(),
                }
            }
            println!("{}", res.iter().join(""));
        }
    }
}
