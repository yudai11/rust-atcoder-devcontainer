use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }

    let circ_num_row = s
        .iter()
        .map(|si| si.iter().filter(|&&c| c == 'o').count())
        .collect::<Vec<_>>();
    let circ_num_column = (0..n)
        .map(|i| s.iter().filter(|&sj| sj[i] == 'o').count())
        .collect::<Vec<_>>();

    let mut ans = 0_usize;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'x' {
                continue;
            }
            ans += (circ_num_column[j] - 1) * (circ_num_row[i] - 1)
        }
    }

    println!("{}", ans);
}
