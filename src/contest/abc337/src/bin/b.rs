use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut state = s[0] as usize - 'A' as usize;
    for &si in s.iter() {
        let x = si as usize - 'A' as usize;
        if state + 1 == x {
            state += 1;
        } else if state == 0 && x == 2 {
            state += 2;
        } else if state != x {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
