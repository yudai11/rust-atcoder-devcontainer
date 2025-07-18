use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize, t: Chars,
        s: [Chars;n]
    }

    let n = t.len();
    let mut ans = vec![];

    for (i, &ref si) in s.iter().enumerate() {
        if n == si.len() {
            let mut check = 0_usize;
            for j in 0..n {
                if t[j] != si[j] {
                    check += 1;
                }
            }
            if check <= 1 {
                ans.push(i + 1);
            }
        }
        if n + 1 == si.len() {
            let mut check = 0_usize;
            let mut j = 0;
            let mut k = 0;
            while j < n && k < n + 1 {
                if t[j] != si[k] {
                    check += 1;
                    k += 1;
                } else {
                    j += 1;
                    k += 1;
                }
            }
            if check <= 1 {
                ans.push(i + 1);
            }
        }
        if n - 1 == si.len() {
            let mut check = 0_usize;
            let mut j = 0;
            let mut k = 0;
            while j < n && k < n - 1 {
                if t[j] != si[k] {
                    check += 1;
                    j += 1;
                } else {
                    j += 1;
                    k += 1;
                }
            }
            if check <= 1 {
                ans.push(i + 1);
            }
        }
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
