use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    let mut emerge_loc = vec![vec![]; 1000_002];
    for (i, &ai) in a.iter().enumerate() {
        emerge_loc[ai].push(i)
    }

    let mut ans = 1000_005_usize;
    for i in 0..1000_002 {
        if emerge_loc[i].len() < 2 {
            continue;
        }
        for j in 0..emerge_loc[i].len() - 1 {
            ans = ans.min(emerge_loc[i][j + 1] - emerge_loc[i][j] + 1);
        }
    }

    if ans > 1000_001 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
