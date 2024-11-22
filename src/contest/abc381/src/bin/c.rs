use proconio::input;
use proconio::marker::Chars;
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
        n: usize,
        s: Chars
    }

    let mut state = s[0];
    let mut conti_one: usize = 0;
    if state == '1' {
        conti_one += 1;
    }
    let mut pre_ans: usize = 1;
    let mut ans: usize = 1;

    for i in 1..n {
        if s[i] == '1' {
            if state == '1' {
                conti_one += 1;
            } else {
                pre_ans = 1;
                conti_one = 1;
            }
            state = '1';
        } else if s[i] == '/' {
            if state == '2' {
                conti_one = 0;
                pre_ans = 1;
            }
            if state == '/' {
                pre_ans = 1;
                conti_one = 0;
            }
            state = '/'
        } else {
            if state == '/' && conti_one > 0 {
                pre_ans = 3;
                conti_one -= 1;
                ans = ans.max(pre_ans);
            }
            if state == '2' && conti_one > 0 && pre_ans > 1 {
                pre_ans += 2;
                conti_one -= 1;
                ans = ans.max(pre_ans);
            }
            state = '2';
        }
    }

    println!("{ans}");
}
