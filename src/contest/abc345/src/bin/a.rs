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

    let mut check = 0;
    let mut cnt = 0;
    for x in s.chars() {
        if check == 0 {
            if x == '<' {
                check = 1;
            } else {
                println!("No");
                return;
            }
        } else if check == 1 {
            if x == '=' {
                check = 2;
            } else {
                println!("No");
                return;
            }
        } else {
            if x == '<' {
                println!("No");
                return;
            }
            if x == '>' {
                cnt += 1;
            }
        }
    }

    if cnt == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
