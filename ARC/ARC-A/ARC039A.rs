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
#[allow(dead_code)]
fn to_num(c: char) -> i64 {
    c as i64 - 48
}


fn main() {
    input!{
        A: chars, B: chars,
    }
    let mut A = A;
    let mut B = B;
    let mut ans = 0;

    for j in 1..3 {
        for i in 0..10 {
            A[j] = std::char::from_digit(i as u32, 10).unwrap();
            let a  = A.into_iter().collect::<String>().parse().unwrap();
            let b  = B.into_iter().collect::<String>().parse().unwrap();
            ans = max(ans, a - b);
        }
        for i in 0..10 {
            B[j] = std::char::from_digit(i as u32, 10).unwrap();
            let a  = A.into_iter().collect::<String>().parse().unwrap();
            let b  = B.into_iter().collect::<String>().parse().unwrap();
            ans = max(ans, a - b);
        }
    }
    for i in 1..10 {
        A[0] = std::char::from_digit(i as u32, 10).unwrap();
        let a  = A.into_iter().collect::<String>().parse().unwrap();
        let b  = B.into_iter().collect::<String>().parse().unwrap();
        ans = max(ans, a - b);
    }
    for i in 1..10 {
        B[0] =  std::char::from_digit(i as u32, 10).unwrap();
        let a  = A.into_iter().collect::<String>().parse().unwrap();
        let b  = B.into_iter().collect::<String>().parse().unwrap();
        ans = max(ans, a - b);
    }
    println!("{}", ans);
}