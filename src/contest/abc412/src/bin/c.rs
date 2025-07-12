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
        t: usize
    }

    let mut res: Vec<isize> = vec![];

    for _i in 0..t {
        input! {
            n: usize,
            mut s: [usize; n]
        }

        let stat = s[0];
        let gol = s[n - 1];

        s.sort();

        let mut cur_height = stat;
        let mut i = 0_usize;
        let mut ans = 0_isize;

        let mut can_domino = true;

        while cur_height * 2 < gol && i < n {
            let mut j = i;

            while j < n && s[j] < cur_height {
                j += 1;
            }

            while j + 1 < n && s[j + 1] <= cur_height * 2 {
                j += 1;
            }

            if s[j] > cur_height && s[j] <= cur_height * 2 {
                cur_height = s[j];
                ans += 1;
                i += 1;
                i = i.max(j);
            } else {
                can_domino = false;
                break;
            }
        }

        if can_domino && cur_height * 2 >= gol {
            res.push(ans + 2);
        } else {
            res.push(-1);
        }
    }

    println!("{}", res.iter().join("\n"));
}
