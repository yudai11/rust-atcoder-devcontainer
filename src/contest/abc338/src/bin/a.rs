use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    if s[0] as u8 >= 'Z' as u8 + 1 {
        println!("No");
        return;
    }

    for (i, &si) in s.iter().enumerate() {
        if i > 0 && (si as u8 + 1 <= 'a' as u8) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
