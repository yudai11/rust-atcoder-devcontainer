use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut ans: usize = 1;
    for (i, si) in s.iter().enumerate() {
        if si == "OR" {
            ans += 1 << (i + 1);
        }
    }

    println!("{ans}");
}
