use proconio::input;

// ニム(Nim)
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    // Nim和
    let nim_sum = a.iter().fold(0_usize, |res, &x| res ^ x);

    if nim_sum != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
