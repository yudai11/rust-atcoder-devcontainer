use num_integer::Roots;
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
        n: usize
    }

    if n < 36 {
        println!("0");
        return;
    }

    // p^8 or p^2 * q^2

    let mut ans: usize = 0;

    let m: usize = n.sqrt() as usize + 2;
    // println!("{m}");
    let mut sieve: Vec<i32> = vec![0; m];

    for i in 2..m {
        if sieve[i] != 0 {
            continue;
        }
        let mut j = i * 2;
        while j < m {
            sieve[j] += 1;
            j += i;
        }
    }

    // for i in 0..20.min(m) {
    //     print!("{} ", sieve[i]);
    // }

    // p^8
    for i in 2..200 {
        if sieve[i] != 0 {
            continue;
        }
        let mut feasi = true;
        let mut x = i;
        for _j in 0..7 {
            x *= i;
            if x > n {
                feasi = false;
                break;
            }
        }

        if feasi {
            ans += 1;
        } else {
            break;
        }
    }

    // p^2 * q^2

    for i in 2..m {
        if sieve[i] != 0 {
            continue;
        }
        if i * i * i * i >= n {
            break;
        }

        for j in i + 1..m {
            if sieve[j] != 0 {
                continue;
            }
            if i * i * j * j <= n {
                ans += 1;
            } else {
                break;
            }
        }
    }

    println!("{ans}");
}
