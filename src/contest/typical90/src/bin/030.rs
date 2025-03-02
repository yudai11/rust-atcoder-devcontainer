use proconio::input;

fn main() {
    input! {
        n: usize, k: usize
    }

    // エラトステネスのふるい
    let mut sieve: Vec<usize> = vec![0; n + 1];
    for i in 2..=n {
        if sieve[i] > 0 {
            continue;
        }
        let mut j = i;
        while j <= n {
            sieve[j] += 1;
            j += i;
        }
    }

    let mut ans = 0_usize;
    for &x in sieve.iter() {
        if x >= k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
