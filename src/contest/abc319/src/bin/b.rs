use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![];
    for i in 0..=n {
        let mut feasi = true;
        for j in 1..10 as usize {
            if n % j != 0 {
                continue;
            }
            if i % (n / j) == 0 {
                feasi = false;
                ans.push((j as u8 + '0' as u8) as char);
                break;
            }
        }

        if feasi {
            ans.push('-')
        }
    }

    println!("{}", ans.iter().join(""));
}
