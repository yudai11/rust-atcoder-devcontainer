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
        n: usize, m: usize,
        mut lines : [(Usize1,Usize1); m]
    }

    // 第0 -> 第1の順に昇順ソート
    lines.sort_by(|x, y| x.0.cmp(&y.0));

    let mut groups = vec![vec![]; n];
    for i in 0..m {
        let (u, v) = lines[i];
        groups[(u + v) % n].push((u, v));
    }

    let mut ans = m * (m - 1) / 2;

    for i in 0..n {
        if groups[i].len() == 0 {
            continue;
        }

        let mut pre_ans = 0_usize;

        // if i == 0 {
        //     for &(u, v) in groups[i].iter() {
        //         if u == n - v {
        //             pre_ans += 1;
        //         }
        //     }

        //     if pre_ans > 1 {
        //         ans -= pre_ans * (pre_ans - 1) / 2;
        //     }

        //     continue;
        // }

        for &(u, v) in groups[i].iter() {
            if u < i {
                if u == i - v {
                    pre_ans += 1;
                }
            } else if u > i {
                if u - i == n - v {
                    pre_ans += 1;
                }
            }
        }

        if pre_ans > 1 {
            ans -= pre_ans * (pre_ans - 1) / 2;
        }
    }

    println!("{}", ans);
}
