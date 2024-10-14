use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [isize; n],
        b: [isize; n]
    }

    let mut test: u64 = 0;

    for i in 0..n {
        test += (a[i] - b[i]).abs() as u64;
    }

    if test <= k && (k - test) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
