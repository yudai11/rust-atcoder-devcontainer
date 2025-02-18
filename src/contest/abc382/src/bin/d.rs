use itertools::Itertools;
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
        n: usize, m: usize
    }

    let mut pre_ans = vec![];
    let mut ans = vec![];
    let mut num_ans = 1_usize;
    for i in 0..n {
        pre_ans.push(1 + i * 10);
    }

    if pre_ans[n - 1] > m {
        return;
    }

    ans.push(pre_ans.clone());
    loop {
        pre_ans[n - 1] += 1;

        // 繰り上げ
        loop {
            let mut can_stop = true;
            for i in (1..n).rev() {
                if pre_ans[i] > m + 10 * i - 10 * (n - 1) {
                    pre_ans[i - 1] += 1;
                    for j in i..n {
                        pre_ans[j] = pre_ans[j - 1] + 10;
                    }
                    can_stop = false;
                }
            }
            if can_stop || pre_ans[0] > m - 10 * (n - 1) {
                break;
            }
        }
        if pre_ans[0] > m - 10 * (n - 1) {
            break;
        }
        ans.push(pre_ans.clone());
        num_ans += 1;
    }

    println!("{}", num_ans);
    for i in 0..num_ans {
        println!("{}", ans[i].iter().join(" "));
    }
}
