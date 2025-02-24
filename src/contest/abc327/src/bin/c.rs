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
        a: [[Usize1;9];9]
    }

    // let mut feasi = true;

    for i in 0..9 {
        // 第i列のチェック
        let mut check = vec![0_usize; 9];
        for j in 0..9 {
            check[a[j][i]] += 1;
        }
        let t = check.iter().fold(1_usize, |res, &x| res * x);
        if t != 1 {
            println!("No");
            return;
        }

        // 第i行のチェック
        let mut check = vec![0_usize; 9];
        for j in 0..9 {
            check[a[i][j]] += 1;
        }
        let t = check.iter().fold(1_usize, |res, &x| res * x);
        if t != 1 {
            println!("No");
            return;
        }
    }

    // 3 times 3 のブロックのチェック
    for i in 0..3 {
        for j in 0..3 {
            let mut check = vec![0_usize; 9];
            for k in 0..3 {
                for l in 0..3 {
                    check[a[3 * i + k][3 * j + l]] += 1;
                }
            }
            let t = check.iter().fold(1_usize, |res, &x| res * x);
            if t != 1 {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
