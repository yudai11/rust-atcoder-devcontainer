use proconio::input;
use proconio::marker::Chars;
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
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;

fn main() {
    input! {
        n: usize, s: Chars,
    }

    let mut num_0 = 0_usize;
    let mut loc_0 = vec![];
    for (i, &si) in s.iter().enumerate() {
        if si == '0' {
            num_0 += 1;
            loc_0.push(i);
        }
    }

    // 0をいくつ左寄せにするかで3分探索
    let mut left = 0_usize;
    let mut right = num_0;
    while left + 2 < right {
        let tri1 = left + (right - left) / 3;
        let tri2 = right - (right - left) / 3;
        if f(tri1, &loc_0, num_0, n) < f(tri2, &loc_0, num_0, n) {
            right = tri2;
        } else {
            left = tri1;
        }
    }

    let mut ans = f(left, &loc_0, num_0, n);
    if left + 1 <= num_0 {
        ans = ans.min(f(left + 1, &loc_0, num_0, n));
    }
    if left + 2 <= num_0 {
        ans = ans.min(f(left + 2, &loc_0, num_0, n));
    }
    println!("{ans}");
}

fn f(tri: usize, loc_0: &Vec<usize>, num_0: usize, n: usize) -> usize {
    let mut res = 0_usize;
    for i in 0..tri {
        res += loc_0[i] - i;
    }
    for i in 1..=(num_0 - tri) {
        res += n - i - loc_0[num_0 - i];
    }
    return res;
}
