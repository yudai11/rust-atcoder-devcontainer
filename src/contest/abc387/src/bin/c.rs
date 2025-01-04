use proconio::input;
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
        l: usize, r: usize
    }

    let mut ans: usize = f(r) - f(l);

    println!("{}", ans);
}

fn f(x: usize) -> usize {
    // lower <= l <= upper <= r
    let mut lower: usize = 1;
    let mut upper: usize = 1;

    while l / lower >= 10 || r / upper >= 10 {
        if l / lower >= 10 {
            lower *= 10;
        }
        if r / upper >= 10 {
            upper *= 10;
        }
    }

    let head_l = l / lower;
    let head_r = r / upper;
    let l_fl = head_l * lower;
    let r_fl = head_r * upper;

    let mut i: usize = 0;
    let mut pow_10: usize = 1;
    while pow_10 <= r_fl {
        let def = pow_10;
        for j in 0..10 {
            if pow_10 > r_fl {
                break;
            }
            let mut plus: usize = 1;
            for k in 0..i {
                plus *= j;
            }
            ans += plus;
            pow_10 += def;
        }

        i += 1
    }

    let mut i: usize = 0;
    let mut pow_10: usize = 1;
    while pow_10 <= l_fl {
        let def = pow_10;
        for j in 0..10 {
            if pow_10 > l_fl {
                break;
            }
            let mut plus: usize = 1;
            for k in 0..i {
                plus *= j;
            }
            ans -= plus;
            pow_10 += def;
        }

        i += 1
    }

    let mut i: usize = 0;
    let mut pow_10: usize = 1;
    while l / pow_10 > 0 {
        if (l / pow_10) % 10 >= head_l {
            break;
        }

        pow_10 *= 10;
    }
    let mut plus = 1;
    for j in 0..i {
        plus *= head_l;
    }
    ans -= plus;

    let mut i: usize = 0;
    let mut pow_10: usize = 1;
    while r / pow_10 > 0 {
        if (r / pow_10) % 10 >= head_r {
            break;
        }
        pow_10 *= 10;
    }
    let mut plus = 1;
    for j in 0..i {
        plus *= head_r;
    }
    ans += plus;
}
