use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        (n, q) : (usize,usize),
        mut a: [usize;n],
    }

    let mut i = 0;

    for _ in 0..q {
        input! {t: u8}
        if t == 1 {
            input! {(x,y): (Usize1,Usize1)}
            let tmp = a[(x + i) % n];
            a[(x + i) % n] = a[(y + i) % n];
            a[(y + i) % n] = tmp;
        }
        if t == 2 {
            input! {(_x,_y): (usize,usize)}
            i += n - 1;
            i %= n;
        }
        if t == 3 {
            input! {(x,_y): (Usize1,usize)}
            println!("{}", a[(x + i) % n]);
        }
    }
}
