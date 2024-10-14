use proconio::input;
use proconio::marker::Chars;
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
        n: usize,
        a: [Chars;n]
    }

    let mut b = a.clone();
    for i in 0..(n / 2) {
        let rot_90 = (i + 1) % 4;
        if rot_90 == 1 {
            for j in (i + 1)..(n - i) {
                b[i][j] = a[n - j - 1][i]; //red
                b[j][n - i - 1] = a[i][j]; //blue
                b[n - i - 1][n - j - 1] = a[j][n - i - 1]; //orange
                b[n - j - 1][i] = a[n - i - 1][n - j - 1]; //black
            }
        } else if rot_90 == 2 {
            for j in (i + 1)..(n - i) {
                b[i][j] = a[n - i - 1][n - j - 1];
                b[j][n - i - 1] = a[n - j - 1][i];
                b[n - i - 1][n - j - 1] = a[i][j];
                b[n - j - 1][i] = a[j][n - i - 1];
            }
        } else if rot_90 == 3 {
            for j in (i + 1)..(n - i) {
                b[i][j] = a[j][n - i - 1];
                b[j][n - i - 1] = a[n - i - 1][n - j - 1];
                b[n - i - 1][n - j - 1] = a[n - j - 1][i];
                b[n - j - 1][i] = a[i][j];
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            print!("{}", b[i][j])
        }
        println!("");
    }
}
