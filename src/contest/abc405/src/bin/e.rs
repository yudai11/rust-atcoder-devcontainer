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
        a: usize, b: usize, c: usize, d: usize
    }

    let mut ans = 0_usize;
    let n = (a + b + c).max(d + c) + 1;

    let mut facts = vec![1_usize; n];
    for i in 1..n {
        facts[i] = facts[i - 1] * i;
        facts[i] %= MOD;
    }

    let mut facts_inv = vec![1_usize; n];
    for i in 2..n {
        facts_inv[i] = mod_inv(facts[i], MOD);
    }

    for i in 0..=c {
        let mut pre_ans = facts[c + d - i - 1] * facts_inv[d - 1];
        pre_ans %= MOD;
        pre_ans *= facts_inv[c - i];
        pre_ans %= MOD;
        pre_ans *= facts[a + b + i];
        pre_ans %= MOD;
        pre_ans *= facts_inv[b];
        pre_ans %= MOD;
        pre_ans *= facts_inv[a + i];
        ans += pre_ans % MOD;
        ans %= MOD;
    }

    println!("{}", ans);
}

// xの逆元を出力
fn mod_inv(x: usize, m: usize) -> usize {
    let x = x as isize;
    let m = m as isize;
    let mut a = vec![x, m];
    let mut b = vec![1_isize, 0_isize];
    while a[0] > 0 && a[1] > 0 {
        let t = a[0] / a[1];
        a[0] -= t * a[1];
        a.reverse();
        b[0] -= t * b[1];
        b.reverse();
    }
    b[0] %= m;
    if b[0] < 0 {
        b[0] += m;
    }
    return b[0] as usize;
}

// fn main() {
//     input! {
//         a: usize, b: usize, c: usize, d: usize
//     }

//     let mut ans = 0_usize;
//     let n = a.max(c) + b + d + 1;

//     let mut facts = vec![1_usize; n];
//     for i in 1..n {
//         facts[i] = facts[i - 1] * i;
//         facts[i] %= MOD;
//     }

//     let mut facts_inv = vec![1_usize; n];
//     for i in 2..n {
//         facts_inv[i] = mod_inv(facts[i], MOD);
//     }

//     for i in 0..=b + d {
//         let mut pre_ans = facts[b + c + d - i - 1] * facts_inv[c - 1];
//         pre_ans %= MOD;
//         pre_ans *= facts_inv[b + d - i];
//         pre_ans %= MOD;
//         pre_ans *= facts[a + i];
//         pre_ans %= MOD;
//         pre_ans *= facts_inv[a];
//         pre_ans %= MOD;
//         pre_ans *= facts_inv[i];
//         ans += pre_ans % MOD;
//     }

//     println!("{}", ans);
// }

// // xの逆元を出力
// fn mod_inv(x: usize, m: usize) -> usize {
//     let x = x as isize;
//     let m = m as isize;
//     let mut a = vec![x, m];
//     let mut b = vec![1_isize, 0_isize];
//     while a[0] > 0 && a[1] > 0 {
//         let t = a[0] / a[1];
//         a[0] -= t * a[1];
//         a.reverse();
//         b[0] -= t * b[1];
//         b.reverse();
//     }
//     b[0] %= m;
//     if b[0] < 0 {
//         b[0] += m;
//     }
//     return b[0] as usize;
// }
