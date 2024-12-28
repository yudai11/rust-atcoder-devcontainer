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
// use ac_library::{Additive, Segtree}; // segtree

fn main() {
    input! {
        _k: usize,
        s: Chars,
        t: Chars
    }

    if s.len() < t.len() {
        let mut ind_s: usize = 0;
        let mut ind_t: usize = 0;
        let mut gap: usize = 0;
        loop {
            if s[ind_s] == t[ind_t] {
                ind_s += 1;
                ind_t += 1;
            } else {
                gap += 1;
                ind_t += 1;
            }

            if ind_t == t.len() || ind_s == s.len() {
                gap += (t.len() - ind_t).max(s.len() - ind_s);
                break;
            }
        }

        if gap <= 1 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if s.len() > t.len() {
        let mut ind_s: usize = 0;
        let mut ind_t: usize = 0;
        let mut gap: usize = 0;
        loop {
            if s[ind_s] == t[ind_t] {
                ind_s += 1;
                ind_t += 1;
            } else {
                gap += 1;
                ind_s += 1;
            }

            if ind_t == t.len() || ind_s == s.len() {
                gap += (t.len() - ind_t).max(s.len() - ind_s);
                break;
            }
        }

        if gap <= 1 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        let mut gap: usize = 0;
        for i in 0..s.len() {
            if s[i] != t[i] {
                gap += 1;
            }
        }

        if gap <= 1 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
