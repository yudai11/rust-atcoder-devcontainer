use proconio::{input, marker::Usize1};
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
        n: usize, k: usize,
        ab: [(Usize1,Usize1); n]
    }

    // field の範囲を決定
    let (a_max, b_max) = ab
        .iter()
        .fold((k, k), |(res1, res2), &x| (res1.max(x.0), res2.max(x.1)));

    let mut field = vec![vec![0_usize; b_max + 1]; a_max + 1];
    for &(a, b) in ab.iter() {
        field[a][b] += 1;
    }

    for i in 0..=a_max {
        for j in 1..=b_max {
            field[i][j] += field[i][j - 1];
        }
    }

    for i in 0..=b_max {
        for j in 1..=a_max {
            field[j][i] += field[j - 1][i];
        }
    }

    let mut ans = 0_usize;

    for i in 0..=a_max - k {
        for j in 0..=b_max - k {
            let mut pre_ans = field[i + k][j + k];
            if i > 0 && j > 0 {
                pre_ans += field[i - 1][j - 1];
                pre_ans -= field[i - 1][j + k];
                pre_ans -= field[i + k][j - 1];
            } else if i > 0 {
                pre_ans -= field[i - 1][j + k];
            } else if j > 0 {
                pre_ans -= field[i + k][j - 1];
            }
            ans = ans.max(pre_ans);
        }
    }

    println!("{}", ans);
}
