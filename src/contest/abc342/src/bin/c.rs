use amplify::confinement::Collection;
use itertools::Itertools;
use proconio::{input, marker::Chars};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
use std::collections::VecDeque;
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
        mut s: Chars,
        q: usize,
        cd: [(char,char); q]
    }

    let mut alph_chabge = vec!['a'; 26];
    let mut alph_change_inv = vec![VecDeque::new(); 26];
    for i in 0..26 as usize {
        alph_chabge[i] = (i as u8 + 'a' as u8) as char;
        alph_change_inv[i].push(i);
    }

    for &(c, d) in cd.iter() {
        if c == d {
            continue;
        }
        while !alph_change_inv[c as usize - 'a' as usize].is_empty() {
            let ind = alph_change_inv[c as usize - 'a' as usize]
                .pop_front()
                .unwrap();
            alph_chabge[ind] = d;
            alph_change_inv[d as usize - 'a' as usize].push(ind);
        }
    }

    for i in 0..n {
        s[i] = alph_chabge[s[i] as usize - 'a' as usize];
    }

    println!("{}", s.iter().join(""));
}
