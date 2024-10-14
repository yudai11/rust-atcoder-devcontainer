use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut residual = 0;
    for i in 0..30 {
        let mut acc = 0; // i桁目の累積XORを保存
        let mut odd = 0;
        for &aj in &a {
            acc ^= (aj >> i) & 1; // aj の i桁目とXOR
            odd += acc;
        }
        residual += (n as i64 + 1 - odd) * odd << i
    }

    residual -= a.iter().sum::<i64>();
    println!("{}", residual)
}
