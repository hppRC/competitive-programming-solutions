#![allow(non_snake_case)]
#[allow(unused_imports)]
use std::io::{self, Write};
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
    ($next:expr, char) => {
        read_value!($next, String).chars().collect::<Vec<char>>()[0]
     };
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
     };
    ($next:expr, isize1) => {
        read_value!($next, isize) - 1
    };
     ($next:expr, $t:ty) => {
         $next().parse::<$t>().expect("Parse error")
    };
}
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}
#[allow(dead_code)]
const MOD: usize = 1000000007;
#[allow(dead_code)]
fn to_num(c: char) -> i64 {
    c as i64 - 48
}

fn main() {
    input!{
        N: usize, Ma: usize, Mb: usize,
        abc: [(usize, usize, usize); N],
    }
    let abrange = 10 * N;
    let mut dp = vec![vec![vec![MOD; abrange+1]; abrange+1]; N+1]; //dp[N][max(a)][max(b)]
    dp[0][0][0] = 0;

    for i in 0..N {
        let a = abc[i].0;
        let b = abc[i].1;
        let c = abc[i].2;
        for j in 0..abrange+1 {
            for k in 0..abrange+1 {
                dp[i+1][j][k] = min(dp[i+1][j][k], dp[i][j][k]);
                if j >= a && k >= b {
                    dp[i+1][j][k] = min(dp[i+1][j][k], dp[i][j - a][k - b] + c);
                }
            }
        }
    }
    let mut ans = MOD;

    for j in 0..abrange {
        for k in 0..abrange {
            if (j * Mb != k * Ma) || (j == 0 && k == 0) {
                continue;
            }
            ans = min(ans, dp[N][j][k]);
        }
    }

    println!("{}", if ans != MOD { ans as isize } else { -1 });
}