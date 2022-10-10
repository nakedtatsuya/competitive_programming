use proconio::input;
use proconio::marker::Bytes;

use std::collections::HashSet;
use std::isize::MIN;
use std::usize::MAX;

use std::collections::VecDeque;

use std::cmp::{max, min};

use itertools::*;

fn main() {
    input! {
        t: usize,
        testcases: [(usize, usize, Bytes); t],
    }

    for testcase in testcases {
        println!("{}", if solve(testcase) { "Yes" } else { "No" });
    }
}

fn solve(testcase: (usize, usize, Vec<u8>)) -> bool {

    let (n, k, a) = testcase;

    let one = a.iter().filter(|x| **x == b'1').count();

    let mut zo = (0, 0);
    let mut ans = 0;
    // 最初の文字からK文字までチェック
    for i in 0..k {
        match a[i] {
            b'0' => { zo.0 += 1; }
            b'1' => { zo.1 += 1; }
            _ => {}
        }
    }
    println!("zo.0: {} zo.1: {} one: {}", zo.0, zo.1, one);

    // K文字がすべて1で全部の１の数と一致する場合ANSを1増やす
    if zo == (0, one) { ans += 1; }

    // Kの単位で1文字ずつチェック
    for i in k..n {
        match a[i] {
            b'0' => { zo.0 += 1; }
            b'1' => { zo.1 += 1; }
            _ => {}
        }

        // Kのスコープから外れる文字をチェック
        match a[i-k] {
            b'0' => { zo.0 -= 1; }
            b'1' => { zo.1 -= 1; }
            _ => {}
        }

        // Kを満たす部分があるときはANSを1増やす
        if zo == (0, one) { ans += 1; }
        println!("zo.0: {} zo.1: {} one: {}", zo.0, zo.1, one);
    }
    
    ans == 1
}