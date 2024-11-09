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
        s: String
    }

    let n = s.len();
    let mut ans = n * (n - 1) / 2;

    let mut alph_num: [usize; 26] = [0; 26];
    for x in s.chars() {
        let index = (x as usize - 'a' as usize) as usize;
        alph_num[index] += 1;
    }

    let mut check = false;
    for &x in alph_num.iter() {
        if x > 1 {
            if !check {
                ans += 1;
                check = true;
            }
            ans -= x * (x - 1) / 2;
        }
    }

    println!("{ans}");
}
