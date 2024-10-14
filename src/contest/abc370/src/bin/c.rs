// use itertools::Itertools;
// use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
// use std::collections::HashMap;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;

fn main() {
    input! {
        mut s: Chars,
        t: Chars
    }

    let n = s.len();
    let x: Vec<_> = (0..n).filter(|&i| t[i] < s[i]).collect();
    let y: Vec<_> = (0..n).filter(|&i| t[i] > s[i]).collect();
    println!("{}", x.len() + y.len());
    for i in x {
        s[i] = t[i];
        println!("{}", s.iter().collect::<String>());
    }
    for &i in y.iter().rev() {
        s[i] = t[i];
        println!("{}", s.iter().collect::<String>());
    }
}
