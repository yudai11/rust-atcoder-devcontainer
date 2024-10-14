use std::collections::VecDeque;

use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        q: usize
    }

    let mut yamahuda: VecDeque<usize> = VecDeque::new();
    let mut ans: Vec<usize> = vec![];
    for _ in 0..q {
        input! {
            t: u8,
            x: usize
        }
        if t == 1 {
            yamahuda.push_front(x);
        } else if t == 2 {
            yamahuda.push_back(x);
        } else if t == 3 {
            ans.push(yamahuda[x - 1]);
        }
    }

    for &x in ans.iter() {
        println!("{}", x);
    }
}
