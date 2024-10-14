use proconio::{input, marker::Usize1};
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
// use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, m: usize,
        ab: [(Usize1,char);m]
    }

    let mut num_baby = vec![0; n];
    for i in 0..m {
        let (a, b) = ab[i];
        if b == 'F' {
            println!("No");
        } else {
            if num_baby[a] == 0 {
                println!("Yes");
            } else {
                println!("No");
            }
            num_baby[a] += 1;
        }
    }
}
