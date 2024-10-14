use proconio::input;
use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut post_loc_of_alph = vec![vec![n; n + 1]; 26];

    for i in (0..n).rev() {
        let int_si = s[i] as i32 - 'a' as i32;
        for j in 0..26 as usize {
            if int_si == j as i32 {
                post_loc_of_alph[j][i] = i;
            } else {
                post_loc_of_alph[j][i] = post_loc_of_alph[j][i + 1];
            }
        }
    }

    let mut ans = String::from("");
    let mut looking = 0;
    for i in 0..k {
        for j in 0..26 as usize {
            let alph = ((j as u8 + 'a' as u8) as char).to_string();
            if n - post_loc_of_alph[j][looking] >= k - i {
                ans.push_str(&alph);
                looking = post_loc_of_alph[j][looking] + 1;
                break;
            }
            // ans.push_str(&alph);
        }
    }

    println!("{}", ans);

    // let x = 'z' as u8 - 'a' as u8;
    // let moji = (x + 'a' as u8) as char;
    // print!("{} is mapped to {}", x, moji);

    // let num = 1; // 対応させたい数字 (1 から 26)
    // let c = (num - 1 + 'a' as u8) as char;
    // println!("{} is mapped to {}", num, c); // 出力: 1 is mapped to a
}
