use proconio::{input, marker::Chars};
use std::collections::HashSet;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        s: [Chars;n]
    }

    let mut name_list = HashSet::new();

    for (i, name) in s.iter().enumerate() {
        if !name_list.contains(name) {
            name_list.insert(name.clone());
            println!("{}", i + 1);
        }
    }
}
