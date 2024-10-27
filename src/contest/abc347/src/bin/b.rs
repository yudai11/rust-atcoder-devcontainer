use std::collections::HashSet;

use proconio::{input, marker::Chars};
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

fn main() {
    input! {
        s: Chars
    }

    let n = s.len();
    let mut ans_set = HashSet::new();

    for i in 0..n {
        let mut sub_str = String::from(s[i]);
        ans_set.insert(sub_str.clone());
        for j in i + 1..n {
            sub_str.push(s[j]);
            ans_set.insert(sub_str.clone());
        }
    }

    println!("{}", ans_set.len());
}
