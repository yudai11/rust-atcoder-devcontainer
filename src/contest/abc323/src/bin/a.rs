use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    if s.len() != 16 {
        println!("Err");
        return;
    }

    for i in (1..16).step_by(2) {
        match s[i] {
            '0' => {}
            '1' => {
                println!("No");
                return;
            }
            _ => unreachable!(),
        }
    }

    println!("Yes");
}
