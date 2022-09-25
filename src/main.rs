use proconio::input;
use std::cmp::{min, max};
use std::usize::MAX;
use itertools::*;
use itertools_num::*;
fn main() {
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

    // println!("{:?}", .sum::<usize>());

    
}
