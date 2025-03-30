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
        n: usize, p: usize, k: usize,
        a: [[isize; n]; n]
    }

    if k == 0 {
        println!("Infinity");
        return;
    }

    // 距離p以下の組の数がk以上になるような最大のxを求める
    let mut left = 0_usize;
    let mut right = p + 10;
    while left < p {
        let mid = left + (right - left) / 2;
        if left == mid {
            break;
        }

        if ways(mid, &a, n, p) >= k {
            left = mid
        } else {
            right = mid
        }
    }

    let upper = left;

    // 距離p以下の組の数がk+1以上になるような最大のxを求める
    let mut left = 0_usize;
    let mut right = p + 10;
    while left < p {
        let mid = left + (right - left) / 2;
        if left == mid {
            break;
        }

        if ways(mid, &a, n, p) >= k + 1 {
            left = mid
        } else {
            right = mid
        }
    }
    let lower = left;

    if lower >= p {
        println!("Infinity")
    } else {
        println!("{}", upper - lower);
    }
}

fn ways(x: usize, a: &Vec<Vec<isize>>, n: usize, p: usize) -> usize {
    // ワーシャルフロイド (warshall_floid) 法
    let mut dist = vec![vec![1000_000_000_000_000_000_usize; n]; n];
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == -1 {
                dist[i][j] = x;
            } else {
                dist[i][j] = a[i][j] as usize;
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }

    // p以下で到達可能な組の数
    let mut res = 0_usize;
    for i in 0..n - 1 {
        for j in i + 1..n {
            if dist[i][j] <= p {
                res += 1;
            }
        }
    }
    return res;
}
