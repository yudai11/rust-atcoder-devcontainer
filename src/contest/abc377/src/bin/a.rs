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

fn main() {
    input! {
        s: Chars
    }

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for &v in s.iter() {
        if v == 'A' {
            a = 1;
        }
        if v == 'B' {
            b = 1;
        }
        if v == 'C' {
            c = 1;
        }
    }

    if a * b * c == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
