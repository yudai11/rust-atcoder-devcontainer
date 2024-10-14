use num_integer::Roots;
use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut ans = 0;
    let mut sieve = vec![0; n + 1];

    for i in 2..=n {
        if sieve[i] != 0 {
            continue;
        }
        let mut j = i;
        while j <= n {
            sieve[j] += 1;
            j += i;
        }
    }

    for i in 2..=n {
        if sieve[i] >= k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
