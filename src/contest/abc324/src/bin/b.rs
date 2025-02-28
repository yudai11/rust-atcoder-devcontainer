use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    // 2除算
    while n > 0 && n % 2 == 0 {
        n /= 2;
    }
    // 3除算
    while n > 0 && n % 3 == 0 {
        n /= 3;
    }

    if n == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
