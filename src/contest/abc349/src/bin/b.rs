use proconio::{input, marker::Chars};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;

fn main() {
    input! {
        s: Chars
    }

    let mut num_alph_appear = [0; 26];
    for &v in s.iter() {
        let index = v as usize - 'a' as usize;
        num_alph_appear[index] += 1;
    }

    for i in 1..=(s.len()) {
        let num_appear_is_i = num_alph_appear
            .iter()
            .fold(0, |sum, &v| if v == i { sum + 1 } else { sum });
        if num_appear_is_i != 0 && num_appear_is_i != 2 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
