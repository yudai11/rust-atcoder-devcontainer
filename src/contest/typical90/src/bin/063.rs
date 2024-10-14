use num::PrimInt;
use proconio::input;
use std::collections::HashSet;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        h:usize, w: usize,
        p: [[usize;w]; h]
    }

    let mut ans: usize = 0;

    for i in 1..(1 << h) {
        let mut index_set: HashSet<usize> = HashSet::new();
        let mut num_full_col: [usize; 80009] = [0; 80009];
        for j in 0..w {
            let mut feasi_full = true;
            let mut first_seen = false;
            let mut first: usize = 0;
            for k in 0..h {
                if (1 << k) & i == 0 {
                    continue;
                }
                if first_seen {
                    if p[k][j] != first {
                        feasi_full = false;
                        break;
                    }
                } else {
                    first = p[k][j];
                    first_seen = true;
                }
            }
            if feasi_full {
                index_set.insert(first);
                num_full_col[first] += 1;
            }
        }

        for &x in index_set.iter() {
            ans = ans.max(num_full_col[x] * i.count_ones() as usize)
        }
    }

    println!("{}", ans);
}
