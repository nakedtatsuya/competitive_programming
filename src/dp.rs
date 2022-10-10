use proconio::input;
use std::cmp::{min, max};
use std::usize::MAX;

/*
    D - Stones
    https://atcoder.jp/contests/abc270/tasks/abc270_d
*/ 
fn game_max_min() {
    input! {
        n:usize,k:usize,
        a: [usize; k]
    }

    let mut dp = vec![vec![0; n+1]; 2];

    for j in 1..=n {
        { //* 先攻 */
            let mut now = 0;
            for x in a.iter() {
                if *x <= j {
                    now = max(now, dp[1][j-x] + x);
                }
            }
            dp[0][j] = now;
        }

        { //* 後攻 */
            let mut now = MAX;
            for x in a.iter() {
                if *x <= j {
                    now = min(now, dp[0][j-x]);
                }
            }
            dp[1][j] = now;
        }
    }
    println!("{}", dp[0][n]);
}
/*
    D - Flip and Adjust 
    https://atcoder.jp/contests/abc271/tasks/abc271_d
*/ 
fn dp_sum_2_patern() {
     input! {
        n:usize, 
        mut s: usize,
        a: [(usize, usize); n],
    }

    let mut dp: Vec<Vec<bool>> = vec![vec![false; s+1]; n+1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..=s {
            if dp[i][j] {
                if j + a[i].0 <= s {
                    dp[i + 1][j + a[i].0] = true;
                }
                if j + a[i].1 <= s {
                    dp[i + 1][j + a[i].1] = true;
                }
            }
        }
    }

    if dp[n][s] {
        println!("Yes");
        let mut t = vec!['H'; n];
        for i in (0..=n-1).rev() {
            println!("==={}", i);
            if s >= a[i].0 && dp[i][s - a[i].0] {
                t[i] = 'H';
                s -= a[i].0;
            } else {
                t[i] = 'T';
                s -= a[i].1;
            }
        }
        println!("{}", t.iter().join(""));
    } else {
       println!("No");
    }
}




use proconio::input;

use std::usize::MAX;

use std::cmp::{min};


fn main() {
    input! {
      n: usize,
      m: usize,
      k: usize,
    }

    let mut g: Vec<(usize, usize, usize)> = vec![(MAX, MAX, MAX); m+1];

    for i in 0..m {
        input! {
          a: usize,
          b: usize,
          c: usize,
        }
        g[i + 1] = (a, b, c);
    }

    input! {
      e: [usize; k],
    }

    // let mut dp = vec![vec![MAX; n + 1]; k + 1];

    // // i まで決めてvにいるときの最小コスト
    // dp[0][1] = 0;

    // for i in 1..=k {
    //     // 使う・使わないのMIN
    //     for v in 1..=n {
    //         // とりあえず前のコストをコピー
    //         dp[i][v] = dp[i-1][v];
    //         // １つ前の状態でvにいることが可能なとき
    //         if dp[i - 1][g[e[i-1]].0] != MAX {
    //             // v から e[i-1] の道を使う場合と使わない場合のMIN 
    //             dp[i][g[e[i-1]].1] = min(dp[i - 1][v], g[e[i-1]].2 + dp[i - 1][g[e[i-1]].0]);
    //         }
    //     }
    // }

    // println!("{}", if dp[k][n] != MAX { dp[k][n].to_string() } else { "-1".to_string() });



    let mut dp = vec![MAX; n + 1];

    // i まで決めてvにいるときの最小コスト
    dp[1] = 0;

    for i in 1..=k {
        // 使う・使わないのMIN
        if dp[g[e[i-1]].0] != MAX {
            // v から e[i-1] の道を使う場合と使わない場合のMIN 
            dp[g[e[i-1]].1] = min(dp[g[e[i-1]].1], g[e[i-1]].2 + dp[g[e[i-1]].0]);
        }
    }

    println!("{}", if dp[n] != MAX { dp[n].to_string() } else { "-1".to_string() });
}
