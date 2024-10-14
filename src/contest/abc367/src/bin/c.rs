use std::process::exit;

use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        r: [usize; n],
    }

    let mut a = vec![1; n + 1];
    a[0] = 0;

    if n % k == 0 {
        for i in 1..=n {
            print!("{} ", a[i]);
        }
        println!("");
    }

    // let slip = n % k;

    // if n > k {
    //     a[n] += k - slip;
    // } else {
    //     a[n] += slip;
    // }
    // for i in (1..=n).rev() {
    //     let advance = a[i] / (r[i - 1] + 1);
    //     a[i - 1] += advance;
    //     a[i] %= r[i - 1] + 1;
    //     a[i] = a[i].max(1);
    // }
    // if a[0] > 0 {
    //     std::process::exit;
    // } else {
    //     for i in 1..=n {
    //         print!("{} ", a[i]);
    //     }
    //     println!("");
    // }

    loop {
        a[n] += 1;
        for i in (1..=n).rev() {
            if i > 0 {
                let advance = a[i] / (r[i - 1] + 1);
                a[i - 1] += advance;
            }

            a[i] %= r[i - 1] + 1;
            a[i] = a[i].max(1);
        }

        if a[0] > 0 {
            break;
        }
        let mut sum_a = 0;
        for i in 1..=n {
            sum_a += a[i];
        }

        if sum_a % k == 0 {
            for i in 1..=n {
                print!("{} ", a[i]);
            }
            println!("");
        }
    }
}
