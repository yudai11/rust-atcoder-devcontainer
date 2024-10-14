use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if b < c {
        if b < a && a < c {
            println!("No");
        } else {
            println!("Yes");
        }
    } else {
        if c < a && a < b {
            println!("Yes")
        } else {
            println!("No");
        }
    }
}
