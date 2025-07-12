use num_integer::Roots;
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

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0_usize;

    for b in 1..=(n.sqrt()) {
        let gap = b * b;
        ans += (n - gap).min(b - 1);
        ans %= MOD;

        // b < k
        let x = n / b;
        ans += (b - 1) * (x - b);
        ans %= MOD;

        // k をbとみる
        let thre = (n + 1) / (b + 1);
        ans += n * (thre - b.min(thre)) - b * (thre - b.min(thre)) * (thre + b + 1) / 2;
        ans += (x + thre - 1) * (x - thre) / 2;
        ans %= MOD;

        // for k in b..=n / b {
        //     if b < k {
        //         let gap = k * b;
        //         let mut plus = (n - gap).min(b - 1) + (n - gap).min(k - 1);
        //         if gap + 1 <= b && b <= gap + 1 + (n - gap).min(b - 1) {
        //             plus -= 1;
        //         }
        //         if gap + 1 <= k && k <= gap + 1 + (n - gap).min(k - 1) {
        //             plus -= 1;
        //         }
        //         ans += plus;
        // ans %= MOD;
        //     }
        // }
    }

    println!("{}", ans);
}
