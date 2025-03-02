use proconio::input;

fn main() {
    input! {
        a: usize, b: usize, c: usize
    }

    let x = c.pow(b as u32);
    if x > a {
        println!("Yes");
    } else {
        println!("No");
    }
}
