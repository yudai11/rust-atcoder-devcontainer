use proconio::input;
use std::process::exit;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut b = vec![0; 2 * n];
    b[0] = a[0];
    for i in 0..(2 * n - 1) {
        b[i + 1] = b[i] + a[(i + 1) % n];
    }

    if b[n - 1] % 10 != 0 {
        println!("No");
        exit(0);
    }
    let sum_over_10 = b[n - 1] / 10;
    match b.binary_search(&sum_over_10) {
        Ok(_) => {
            println!("Yes");
            exit(0);
        }
        Err(_) => {}
    }
    for i in 0..n {
        match b.binary_search(&(sum_over_10 + b[i])) {
            Ok(_) => {
                println!("Yes");
                exit(0);
            }
            Err(_) => {}
        }
    }

    println!("No");
}
