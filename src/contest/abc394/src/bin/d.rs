use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }
    let n = s.len();
    if n % 2 != 0 {
        println!("No");
        return;
    }

    let mut seen_list = vec![];
    for i in 0..n {
        match s[i] {
            '(' | '[' | '<' => {
                seen_list.push(s[i]);
            }
            ')' => {
                if let Some(last) = seen_list.pop() {
                    if last != '(' {
                        println!("No");
                        return;
                    }
                } else {
                    println!("No");
                    return;
                }
            }
            ']' => {
                if let Some(last) = seen_list.pop() {
                    if last != '[' {
                        println!("No");
                        return;
                    }
                } else {
                    println!("No");
                    return;
                }
            }
            '>' => {
                if let Some(last) = seen_list.pop() {
                    if last != '<' {
                        println!("No");
                        return;
                    }
                } else {
                    println!("No");
                    return;
                }
            }
            _ => unreachable!(),
        }
    }

    if seen_list.len() == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
