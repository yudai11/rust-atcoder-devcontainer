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
        n: usize, _s: u128,
        _a: [usize;n]
    }

    let mut a: Vec<usize> = vec![0; 2 * n];
    for i in 0..2 * n {
        a[i] = _a[i % n];
    }

    let sum = _a.iter().sum::<usize>();
    let s = (_s % sum as u128) as usize;
    if s == 0 {
        println!("Yes");
        return;
    }

    // しゃくとりをする
    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut cur: usize = a[0];

    while right + 1 < 2 * n || left < right {
        if cur < s && right + 1 < 2 * n {
            cur += a[right + 1];
            right += 1;
        } else if cur > s {
            cur -= a[left];
            left += 1
        } else if cur < s {
            println!("No");
            return;
        }

        if cur == s {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
