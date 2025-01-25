use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;
// use proconio::marker::Chars;
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
        a: [usize;n]
    }

    let mut ans: usize = 0;
    let mut seen = HashSet::new();
    // 0,n
    let sum = a.iter().fold(0_usize, |sum, &x| sum + x);
    let xor_sum = a.iter().fold(0_usize, |res, &x| res ^ x);
    // seen.insert(sum);
    // seen.insert(xor_sum);

    // 1~l個選んでそれを圧縮するかそれ以外を圧縮するかを考える．
    let l = n / 2;
    for i in 0..=l {
        for com in (0..n).combinations(i) {
            let mut chosen_sum: usize = 0;
            let mut other_sum: usize = sum;
            let mut chosen_xor: usize = 0;
            let mut other_xor: usize = xor_sum;
            for k in com {
                chosen_sum += a[k];
                other_sum -= a[k];
                chosen_xor ^= a[k];
                other_xor ^= a[k];
            }

            if !seen.contains(&(chosen_sum ^ other_xor)) {
                ans += 1;
                seen.insert(chosen_sum ^ other_xor);
            }
            if !seen.contains(&(chosen_xor ^ other_sum)) {
                ans += 1;
                seen.insert(chosen_xor ^ other_sum);
            }
        }
    }

    // let l = n / 2;

    // if n % 2 == 0 {
    //     for com in (0..n).combinations(l) {
    //         let mut chosen_sum: usize = 0;
    //         let mut other_sum: usize = sum;
    //         let mut chosen_xor: usize = 0;
    //         let mut other_xor: usize = xor_sum;
    //         for k in com {
    //             chosen_sum += a[k];
    //             other_sum -= a[k];
    //             chosen_xor ^= a[k];
    //             other_xor ^= a[k];
    //         }

    //         if !seen.contains(&(chosen_sum ^ other_xor)) {
    //             ans += 1;
    //             seen.insert(chosen_sum ^ other_xor);
    //         }
    //         // if !seen.contains(&(chosen_xor ^ other_sum)) {
    //         //     ans += 1;
    //         //     seen.insert(chosen_xor ^ other_sum);
    //         // }
    //     }
    // }

    println!("{ans}");
}
