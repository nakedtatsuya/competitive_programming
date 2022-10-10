use proconio::input;

use std::collections::HashSet;
use std::usize::MAX;
use std::isize::MIN;

use std::collections::VecDeque;

use std::cmp::{min, max};

use itertools::*;

fn main() {
    input! {
      n: usize,
      m: usize,
      k: usize,
    }

    let mut g: Vec<Vec<usize>> = vec![vec![]; n+1];


    for _ in 0..m {
      input! {
        a: usize,
        b: usize,
      }
      g[a].push(b);
      g[b].push(a);
    }


    input! {
        a: [usize; n],
    }

    input! {
        b: [usize; k],
    }

    let mut used: Vec<bool> = vec![false; n+1];

    let mut stack: VecDeque<usize> = VecDeque::new();
    let mut routes: HashSet<VecDeque<usize>> = HashSet::new();
    // g[1].reverse();
    dfs_sinple_path(&mut used, &g, &mut stack, 1, n, &mut routes);


    println!("{:?}", routes);


}

fn dfs_sinple_path(used: &mut Vec<bool>, g: &Vec<Vec<usize>>, stack: &mut VecDeque<usize>,  cur: usize, to: usize, routes: &mut HashSet<VecDeque<usize>>) {
  stack.push_back(cur);


  if cur != to {
    used[cur] = true;
  }

  for next in g[cur].iter() {
    if used[*next] {
      continue;
    } else {
      dfs_sinple_path(used, g, stack, *next, to, routes);
    }

    if stack[stack.len() - 1] == to {
      routes.insert(stack.clone());
      break;
    } else {
      stack.pop_back();
    }
  }
}

