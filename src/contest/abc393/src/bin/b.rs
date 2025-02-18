use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars,
    }

    let mut loc_a = vec![];
    let mut loc_b = vec![];
    let mut loc_c = HashSet::new();
    for (i, &si) in s.iter().enumerate() {
        if si == 'A' {
            loc_a.push(i);
        }
        if si == 'B' {
            loc_b.push(i);
        }
        if si == 'C' {
            loc_c.insert(i);
        }
    }

    let mut ans = 0_usize;

    for &ai in loc_a.iter() {
        for &bi in loc_b.iter() {
            if ai < bi && loc_c.contains(&(2 * bi - ai)) {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}
