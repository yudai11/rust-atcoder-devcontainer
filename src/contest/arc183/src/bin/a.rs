use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use petgraph::unionfind::UnionFind;

fn main() {
    input! {
         n: usize,
         k: usize
    }

    if n == 1 {
        for i in 0..k {
            print!("1 ");
        }
    } else if n % 2 == 0 {
        let mut a = vec![k; n];
        print!("{} ", n / 2);
        a[(n / 2) - 1] -= 1;
        for i in (0..n).rev() {
            while a[i] > 0 {
                print!("{} ", i + 1);
                a[i] -= 1;
            }
        }
    } else {
        let mut a = vec![k; n];
        let mut com = k;
        let mut x = (n / 2) + 1;
        for _i in 0..com {
            print!("{} ", x);
        }
        a[x - 1] -= com;
        print!("{} ", n / 2);
        a[(n / 2) - 1] -= 1;
        for i in (0..n).rev() {
            while a[i] > 0 {
                print!("{} ", i + 1);
                a[i] -= 1;
            }
        }
    }
}
