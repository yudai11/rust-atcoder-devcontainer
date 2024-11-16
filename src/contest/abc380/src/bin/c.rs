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
        n: usize, k: usize,
        s: Chars
    }

    let mut t = vec![];
    let mut cnt: usize = 0;
    let mut state = 0;
    let mut index: usize = 0;

    for i in 0..n {
        t.push(s[i].clone());

        if state == 0 {
            // 直前に0を観測
            if s[i] == '1' {
                state = 1;
            }
        } else {
            // 直前に1を観測
            if s[i] == '0' {
                state = 0;
                cnt += 1;
            }

            if cnt == k - 1 {
                t.pop();
                index = i;
                break;
            }
        }
    }

    let mut cnt_0 = 0;

    for i in index..n {
        if cnt == k - 1 {
            if state == 0 {
                if s[i] == '0' {
                    cnt_0 += 1;
                } else {
                    t.push(s[i]);
                    state = 1;
                    cnt += 1;
                }
            }
        } else if cnt == k {
            t.push(s[i]);
            if s[i] == '0' {
                state = 0;
                cnt += 1;
                while cnt_0 > 0 {
                    t.push('0');
                    cnt_0 -= 1;
                }
            }
        } else {
            t.push(s[i].clone());
        }
    }

    while cnt_0 > 0 {
        t.push('0');
        cnt_0 -= 1;
    }

    for &x in t.iter() {
        print!("{x}");
    }
}
