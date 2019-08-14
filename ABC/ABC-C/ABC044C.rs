#![allow(non_snake_case)]
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
     };
     ($next:expr, $t:ty) => {
         $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(dead_code)]
const MOD: u64 = 1000000007;


fn main() {
    input!{
        N: usize, A: i64,
        X: [i64; N]
    }
    let Y = X.into_iter().map(|x| x - A).collect::<Vec<i64>>();
    let mut dp: Vec<Vec<i64>> = vec![vec![0; 5001]; N+1];

    dp[0][2500] = 1;

    for i in 1..N+1 {
        let yi = Y[i-1];
        for j in 0..(5001 as usize) {
            let sum = j as i64 - yi;
            if sum < 0 || sum > 5000 {
                dp[i][j] += dp[i-1][j]
            } else if sum >= 0 && sum <= 5000 {
                dp[i][j] += dp[i-1][j] + dp[i-1][sum as usize];
            }
        }
    }
    println!("{}", dp[N][2500] - 1);

}