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
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
     };
     ($next:expr, $t:ty) => {
         $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(dead_code)]
const MOD: u64 = 1000000007;

fn digit_sum(n: i64, b: i64) -> i64 {
    if n < b {
        n
    } else {
        digit_sum((n as f64/b as f64).floor() as i64, b) + (n % b)
    }
}


fn main() {
    input!{
        n: usize,
        s: usize,
    }

    let ce = (n as f64).sqrt().ceil() as usize;

    if n == s {
        println!("{}", n+1);
    } else if n < s {
        println!("-1");
    } else {

        let mut ans: i64 = -1;

        for p in (1..ce+1).rev() {
            if (n - s) % p != 0 {
                continue;
            }
            let b = (n - s) / p + 1;
            if digit_sum(n as i64, b as i64) == s as i64 {
                ans = b as i64;
                break;
            }
        }

        for b in 2..ce+1 {
            let ret = digit_sum(n as i64, b as i64);
            if ret == s as i64 {
                ans = b as i64;
                break;
            }
        }

        println!("{}", ans);

    }


}