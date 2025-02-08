use num_integer::Roots;
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
        k: usize,
    }

    let m: usize = k.sqrt() as usize + 2;

    let mut is_prime_k = true;

    let mut sieve: Vec<usize> = vec![0; m];
    let mut prime_factors = vec![];

    for i in 2..m {
        if sieve[i] != 0 {
            continue;
        }
        if k % i == 0 {
            is_prime_k = false;
            let mut l = i;
            let mut cnt_pfac = 1_usize;
            loop {
                l *= i;
                cnt_pfac += 1;
                if k % l != 0 || l > k {
                    prime_factors.push(cnt_pfac);
                    break;
                }
            }
        }
        let mut j = i * 2;
        while j < m {
            sieve[j] += 1;
            j += i;
        }
    }

    let ans = prime_factors.iter().fold(1_usize, |prod, &x| prod * x);

    println!("{}", ans);
}
