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
        mut a: [usize;n]
    }
    let mut ans = 0_usize;
    let mut how_many_factors = [0_usize; 2000_01];
    for i in 0..n {
        let mut new_ai = a[i];
        for j in 2..a[i] {
            let s = j * j;
            if s > new_ai {
                break;
            }
            while new_ai % s == 0 {
                new_ai /= s;
            }
        }
        a[i] = new_ai;
        how_many_factors[new_ai] += 1;
    }

    let mut seen_zero = 0_usize;
    for &ai in a.iter() {
        if ai == 0 {
            seen_zero += 1;
            ans += n - seen_zero;
        } else {
            how_many_factors[ai] -= 1;
            ans += how_many_factors[ai];
        }
    }
    println!("{}", ans);
}
