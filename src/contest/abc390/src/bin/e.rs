use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// 優先度付きのque, peek,popで最大値を散り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()で最小値を取れる)
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree

fn main() {
    input! {
        n: usize, x: usize,
        vac: [(Usize1,usize,usize);n]
    }

    let mut dp: Vec<Vec<usize>> = vec![vec![1000_000_000_000; 3]; x + 1];
    for i in 0..3 {
        dp[0][i] = 0;
    }
    for i in 0..n {
        let (v, a, c) = vac[i];
        for j in 0..=x - c {
            for k in 0..3 {
                if dp[j][k] < 1000_000_00_000 {
                    if dp[j + c][k] > 1000_000_00_000 {
                        dp[j + c][v] = dp[j][k] + a;
                    } else {
                        dp[j + c][v] = dp[j + c][v].max(dp[j][k] + a);
                    }
                }
            }
        }
    }

    // 二分探索
    let mut right = usize::MAX;
    let mut left: usize = 0;
    loop {
        let mid = left + (right - left) / 2;
        if left == mid {
            break;
        }
        if can(x, mid, &dp) {
            left = mid
        } else {
            right = mid
        }
    }

    println!("{left}");
}

fn can(x: usize, y: usize, dp: &Vec<Vec<usize>>) -> bool {
    let mut mins = [10000_usize; 3];
    for i in 0..x + 1 {
        for j in 0..3 {
            if dp[i][j] >= y && dp[i][j] < 1000_000_00_000 {
                mins[j] = mins[j].min(i);
            }
        }
    }

    return mins.iter().fold(0_usize, |sum, &x| sum + x) <= x;
}
