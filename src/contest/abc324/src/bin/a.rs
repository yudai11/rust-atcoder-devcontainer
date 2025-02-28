use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    for &ai in a.iter() {
        if ai != a[0] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
