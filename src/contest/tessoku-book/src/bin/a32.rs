use proconio::input;

// 石取りゲーム
fn main() {
    input! {
        n: usize, a: usize, b: usize,
    }

    let mut can_win = vec![false; n + 1];
    let l = a.min(b);
    for i in l..=n {
        if i >= a && !can_win[i - a] {
            can_win[i] = true;
        }
        if i >= b && !can_win[i - b] {
            can_win[i] = true;
        }
    }

    if can_win[n] {
        println!("First");
    } else {
        println!("Second");
    }
}
