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
        b: u128
    }

    for i in 1..30 {
        let check = pow_mine(i as u128, i);
        if check > b {
            break;
        } else if check == b {
            println!("{}", i);
            return;
        }
    }

    println!("-1")
}

fn pow_mine(c: u128, b: usize) -> u128 {
    let mut b_bi_list: Vec<bool> = vec![false; 5];
    for i in 0..5 {
        if (b >> i) & 1 == 1 {
            b_bi_list[i] = true;
        }
    }
    let mut ans: u128 = 1;
    let mut c_pow_2i: u128 = c;

    for i in 0..5 {
        ans *= if b_bi_list[i] { c_pow_2i } else { 1 };
        c_pow_2i *= c_pow_2i;
    }

    ans
}
