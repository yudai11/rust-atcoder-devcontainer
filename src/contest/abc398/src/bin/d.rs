use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::HashSet;
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
        n: usize, r: isize, c: isize,
        s: Chars
    }

    // 自分の位置 風の反対方向に動く
    let mut loc = (0_isize, 0_isize);

    let mut field = vec![HashSet::new(); 8000_09];
    field[4000_04].insert(0_isize);
    let mut ans = vec![0_usize; n];

    for (i, &si) in s.iter().enumerate() {
        match si {
            'N' => {
                loc.1 -= 1;
            }
            'S' => {
                loc.1 += 1;
            }
            'E' => {
                loc.0 += 1;
            }
            'W' => {
                loc.0 -= 1;
            }
            _ => unreachable!(),
        }

        // 原点を灯す
        field[(4000_04 - loc.0) as usize].insert(-loc.1);

        if field[(4000_04 + c - loc.0) as usize].contains(&(r - loc.1)) {
            ans[i] = 1;
        }
    }

    println!("{}", ans.iter().join(""));
}
