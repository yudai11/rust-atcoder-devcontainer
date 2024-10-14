use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        a: [u128;3]
    }

    // let mut b: Vec<u128> = a.clone();
    let gcd = gcd_2(a[0], gcd_2(a[1], a[2]));
    // println!("{}", gcd);
    let mut num_cut: u128 = 0;

    for i in 0..3 {
        num_cut += a[i] / gcd - 1;
    }

    println!("{}", num_cut);
}

fn gcd_2(x: u128, y: u128) -> u128 {
    let mut a = vec![x, y];
    loop {
        a.sort();
        if a[0] <= 1 {
            return 1;
        }
        let m = a[1] % a[0];
        if m == 0 {
            return a[0];
        }
        // if m == 1 {
        //     return 1;
        // }
        a[1] = m;
        // a[2] = l;
    }
}
