use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        a: i128,
        b: i128
    }

    let c = gcd(a, b);
    if a / c * b > 1000000000000000000 as i128 {
        println!("Large");
    } else {
        println!("{}", (a / c * b))
    }
}

fn gcd(x: i128, y: i128) -> i128 {
    let mut a: Vec<i128> = vec![x, y];
    loop {
        a.sort();
        if a[0] <= 1 {
            return 1;
        }
        let m = a[1] % a[0];
        if m == 0 {
            return a[0];
        }
        a[1] = m;
    }
}
