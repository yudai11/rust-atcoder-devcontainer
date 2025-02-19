use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut cnt = vec![0_usize; 26];
    for &si in s.iter() {
        cnt[si as usize - 'a' as usize] += 1;
    }
    let mut ans = 0_usize;
    let mut max_val = 0_usize;
    for (i, &x) in cnt.iter().enumerate() {
        if x > max_val {
            ans = i;
            max_val = x;
        }
    }

    println!("{}", (ans as u8 + 'a' as u8) as char);
}
