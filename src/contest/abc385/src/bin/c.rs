use proconio::input;
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
        n: usize,
        h: [usize;n]
    }

    let mut ans: usize = 1;

    let mut place_buildings = vec![vec![]; 3003];
    for i in 0..n {
        place_buildings[h[i]].push(i);
    }

    //各高さごとに最長の等差列を見つける．
    for i in 0..3003 {
        if place_buildings[i].len() < 1 {
            continue;
        }

        let a = place_buildings[i].clone();

        let m = a.len();
        let mut dp = vec![vec![1; 3001]; m];
        let mut max_length = 1;

        for i in 1..m {
            for j in 0..i {
                let diff = a[i] - a[j];
                dp[i][diff] = dp[j][diff] + 1;
                max_length = max_length.max(dp[i][diff]);
            }
        }

        ans = ans.max(max_length);
    }

    println!("{ans}");
}
