use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize, m: usize,
        s: Chars,
        t: Chars
    }

    let gap = m - n;

    let mut feasi_1 = true;
    for i in 0..n {
        if s[i] != t[i] {
            feasi_1 = false;
            break;
        }
    }

    let mut feasi_2 = true;
    for i in 0..n {
        if s[i] != t[i + gap] {
            feasi_2 = false;
            break;
        }
    }

    if feasi_1 && feasi_2 {
        println!("0");
    } else if feasi_1 {
        println!("1");
    } else if feasi_2 {
        println!("2");
    } else {
        println!("3");
    }
}
