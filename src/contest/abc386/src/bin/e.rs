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

// 注意　余事象に気をつけよう

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize;n]
    }

    // 全コンビネーションの上手い見方がある．
    // xorの更新は高々O(log(A_i)) = 60

    if k <= n - k {
        let res = combinations(n, k, &a, 0);
        let ans = res.iter().fold(usize::MIN, |max, &x| max.max(x));
        println!("{ans}");
    } else {
        let init = a.iter().fold(0 as usize, |res, &x| res ^ x);
        let res = combinations(n, n - k, &a, init);
        let ans = res.iter().fold(usize::MIN, |max, &x| max.max(x));
        println!("{ans}");
    }
}

fn combinations(n: usize, k: usize, a: &Vec<usize>, init: usize) -> Vec<usize> {
    let mut result = Vec::new();
    let mut current = Vec::new();

    fn backtrack(
        start: usize,
        n: usize,
        k: usize,
        current: &mut Vec<usize>,
        ans: usize,
        result: &mut Vec<usize>,
        a: &Vec<usize>,
    ) {
        if current.len() == k {
            result.push(ans);
            return;
        }

        for i in start..n {
            current.push(i);
            let ans = ans ^ a[i];
            backtrack(i + 1, n, k, current, ans, result, a);
            let j = current.pop().unwrap();
            let ans = ans ^ a[j];
        }
    }

    backtrack(0, n, k, &mut current, init, &mut result, a);
    result
}
