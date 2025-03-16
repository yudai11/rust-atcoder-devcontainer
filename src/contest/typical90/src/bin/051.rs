use proconio::input;
use superslice::Ext; // for use of lowerbound upperbound method of vetor

// 半分全列挙
fn main() {
    input! {
        n: usize, k: usize, p: usize,
        a: [usize; n]
    }

    let mut group1 = vec![];
    let mut group2 = vec![];

    for i in 0..n / 2 {
        group1.push(a[i]);
    }
    for i in n / 2..n {
        group2.push(a[i]);
    }

    let list_1 = make_list(&group1, p, k);
    let list_2 = make_list(&group2, p, k);

    let mut ans = 0_usize;
    for i in 0..=k {
        for &x in list_1[i].iter() {
            ans += list_2[k - i].lower_bound(&(p + 1 - x));
        }
    }

    println!("{}", ans);
}

fn make_list(b: &Vec<usize>, p: usize, k: usize) -> Vec<Vec<usize>> {
    let l = b.len();
    // 商品を i つ選ぶときの値段を格納
    let mut res = vec![vec![]; k + 1];
    for i in 0..(1 << l) {
        let mut sum = 0_usize;
        let mut pick_cnt = 0_usize;
        for j in 0..l {
            if (i >> j) & 1 == 1 {
                pick_cnt += 1;
                sum += b[j];
            }
        }
        if pick_cnt > k || sum > p {
            continue;
        }
        res[pick_cnt].push(sum);
    }

    for i in 0..=k {
        res[i].sort();
    }

    return res;
}
