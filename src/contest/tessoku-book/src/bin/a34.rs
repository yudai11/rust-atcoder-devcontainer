use proconio::input;

// Grundy数とは
// 盤面 x のGrundy数 y は次で定まる．盤面 x から遷移可能な盤面を x_1,...,x_k としそれぞれのGrundy数を y_1,...,y_k とすると，y は {y_1,...,y_k} 以外の最小の非負整数である．
// 各山のGrundy数のXOR和が0ならば後手必勝，!=0 ならば先手必勝

fn main() {
    input! {
        n: usize, x: usize, y: usize,
        a: [usize; n]
    }

    let mut grundy_numbers = vec![0_usize; 1000_01];
    for i in x..=1000_00 {
        let mut seen = vec![];
        seen.push(grundy_numbers[i - x]);
        if y <= i {
            seen.push(grundy_numbers[i - y]);
        }
        seen.sort();
        for j in 0..3 {
            if seen.contains(&j) {
                continue;
            } else {
                grundy_numbers[i] = j;
                break;
            }
        }
    }

    let grundy_sum = (0..n)
        .collect::<Vec<usize>>()
        .iter()
        .fold(0_usize, |res, &i| res ^ grundy_numbers[a[i]]);

    if grundy_sum != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
