use proconio::input;
use std::cmp::{min, max};

/*
    E - Apple Baskets on Circle
    https://atcoder.jp/contests/abc270/tasks/abc270_e
*/
fn simulation_baskets_on_ircle() {
    input! {
        n:usize, mut k:usize,
        mut a: [usize; n]
    }

    let mut event = a.clone();

    event.sort();

    // 周回の数
    let mut pre: usize = 0;

    // 一周のループ
    for i in 0..n {
        // １周で減る量
        let r: usize = n-i;

        // 最初のかごが売り切れるまでに消費する量
        let num: usize = r*(event[i]-pre);

        // Kに収まるなら消費
        if num <= k && i != n-1 {
            k -= num;
        } else {
            // 周回の途中
            pre += k/r;
            k%=r;
            for j in 0..n {
                if a[j] <= pre {
                    a[j] = 0;
                } else {
                    a[j] -= pre;
                }
            }
            for j in 0..n {
                if k > 0 && a[j] > 0 {
                    a[j] -= 1;
                    k -= 1;
                }
            }
            break;
        }
        pre = event[i];
    }
    a.iter().for_each(|v| print!("{} ", v));
}

/*
    C - Manga
    https://atcoder.jp/contests/abc271/tasks/abc271_c
*/
fn simulation_baskets_on_ircle() {
    input! {
        n:usize, 
        mut a: [usize; n],
    }
    let count = a.len();
    let uniq: HashSet<usize> = a.into_iter().collect();

    let diff = count - uniq.len();

    a = [uniq.iter().cloned().collect(), vec![MAX; diff]].concat();

    a.sort();

    let mut have_mangas: VecDeque<usize> = a.into_iter().collect();
    let mut stack: Vec<usize> = vec![0];

    loop {

        if have_mangas.len() <= 0 {
            break;
        }

        if have_mangas[0] == stack[stack.len()-1]+1 {
            have_mangas.pop_front();
            stack.push(stack[stack.len()-1]+1);
            continue;
        }

        if have_mangas.len() == 1 {
            break;
        }

        if have_mangas.len() >= 2 {
            have_mangas.pop_back();
            have_mangas.pop_back();
            stack.push(stack[stack.len()-1]+1);
            continue;
        }
    }

    println!("{}", stack.len()-1);    
}

