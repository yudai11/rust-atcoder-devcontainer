use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    for i in 0..n - 1 {
        if a[i] >= a[i + 1] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
