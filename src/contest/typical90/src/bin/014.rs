use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        _a: [i64;n],
        _b: [i64;n],
    }

    let mut a = _a;
    let mut b = _b;

    a.sort();
    b.sort();

    let mut ans: u128 = 0;

    for i in 0..n {
        ans += (a[i] - b[i]).abs() as u128;
    }

    println!("{}", ans);
}
