use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
use std::collections::BinaryHeap;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;
// use superslice::Ext; // for use of lowerbound upperbound method of vetor

fn main() {
    input! {
        n: usize, k: usize,
        c: [[usize;n];n],
        q: usize,
        st: [(Usize1,Usize1);q]
    }

    for &(s, t) in st.iter() {
        let mut cost_mat = vec![vec![0_usize; n + 2]; n + 2];
        // コスト行列の作成
        for i in 0..k + 2 {
            for j in 0..k + 2 {
                let mut x = i;
                let mut y = j;
                if i == k {
                    x = s;
                } else if i == k + 1 {
                    x = t;
                }
                if j == k {
                    y = s;
                } else if j == k + 1 {
                    y = t;
                }
                cost_mat[i][j] = c[x][y];
            }
        }

        let res = prim(&cost_mat, k + 2);
        println!("{}", res);
    }
}

// 余分な辺も含んで良いのでprim法はNG
fn prim(c: &Vec<Vec<usize>>, n: usize) -> usize {
    let mut ans = 0_usize;
    let mut seen = vec![false; n];
    let mut queue: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    seen[0] = true;
    for i in 0..n {
        if seen[i] {
            continue;
        }
        queue.push((Reverse(c[i][0]), i));
    }
    while let Some(p) = queue.pop() {
        if seen[p.1] {
            continue;
        }
        let Reverse(cost) = p.0;
        ans += cost;
        seen[p.1] = true;
        for i in 0..n {
            if seen[i] {
                continue;
            }
            queue.push((Reverse(c[p.1][i]), i));
        }
    }
    return ans;
}
