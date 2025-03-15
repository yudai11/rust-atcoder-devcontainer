use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n]
    }

    let b = press_coordinate(&a);
    let m = n + 1;
    let mut flag = vec![0_usize; m];

    // 尺取り法
    let mut ans = 0_usize;
    let mut left = 0_usize;
    let mut right = 0_usize;
    flag[b[right]] += 1;
    let mut cur_kind = 1_usize;

    while right < n - 1 {
        right += 1;
        if flag[b[right]] == 0 {
            cur_kind += 1;
        }
        flag[b[right]] += 1;

        while cur_kind > k && left <= right {
            flag[b[left]] -= 1;
            if flag[b[left]] == 0 {
                cur_kind -= 1;
            }
            left += 1;
        }
        ans = ans.max(right + 1 - left);
    }

    println!("{}", ans);
}

fn press_coordinate(list: &Vec<usize>) -> Vec<usize> {
    let mut copy_set = HashSet::new();
    for &v in list {
        copy_set.insert(v);
    }

    let mut copy_list = vec![];
    for &x in copy_set.iter() {
        copy_list.push(x);
    }
    copy_list.sort();

    let mut press_map: HashMap<usize, usize> = HashMap::new();
    for (i, &k) in copy_list.iter().enumerate() {
        press_map.insert(k, i);
    }

    let mut pressed_list: Vec<usize> = vec![];
    for x in list {
        pressed_list.push(press_map[x]);
    }

    pressed_list
}
