use proconio::input;
// use proconio::marker::Chars;
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
        n: usize, m: usize, k: usize,
        a: [usize;n]
    }

    let sum_vote = a.iter().fold(0, |sum, &x| sum + x) as usize;
    let mut a_sorted = a.clone();
    a_sorted.sort();

    let mut cumsum_a: Vec<usize> = vec![0; n];
    cumsum_a[0] = a_sorted[0];
    for i in 0..n - 1 {
        cumsum_a[i + 1] = cumsum_a[i] + a_sorted[i + 1];
    }

    let mut ans = vec![-1; n];
    // 2butan
    for i in 0..n {
        let ai = a[i];
        let mut max: isize = (k - sum_vote + 1) as isize;
        let mut min: isize = -1;
        // let mut ans: u64 = 0;

        loop {
            let test = min + (max - min) / 2;
            if test == min {
                break;
            }
            if is_larger_than_ans(&a_sorted, &cumsum_a, ai, test, sum_vote, k, m, n) {
                max = test;
            } else {
                min = test;
            }
        }

        ans[i] = min;
    }

    for x in ans {
        print!("{x} ");
    }
}

fn is_larger_than_ans(
    a: &Vec<usize>,
    cumsum: &Vec<usize>,
    ai: usize,
    test: isize,
    sum_vote: usize,
    k: usize,
    m: usize,
    n: usize,
) -> bool {
    let x = match a.binary_search(&(ai + test as usize)) {
        Ok(x) => x,
        Err(x) => x,
    };
    // ai+test より小さいのがxつある

    if x < n - m {
        return false;
    }

    let near = m + x + 1 - n;
    let gap = (ai + test as usize + 1) * near - (cumsum[x - 1] - cumsum[x - near - 1]);
    let residual = k - sum_vote - test as usize;

    if gap > residual {
        return true;
    } else {
        return false;
    }
}
