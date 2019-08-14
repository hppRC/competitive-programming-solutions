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

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}
impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

#[allow(dead_code)]
const MOD: u64 = 1000000007;
#[allow(dead_code)]
fn to_num(c: char) -> i64 {
    c as i64 - 48
}


fn main() {
    input!{
        N: usize, D: usize,
        R: [usize; N]
    }
    let mut R = R;
    R.sort();

    let mut dp: Vec<usize> = vec![0; N+1];

    dp[3] = if R[2] - R[0] <= D {1} else {0};

    for i in 3..N {
        let rate = R[i];
        let low = if rate <= D { 0 } else { rate - D };
        let l = R.lower_bound(&low);
        let ret = if i != l { (i - l) * (i - l - 1) / 2} else { 0 };
        dp[i+1] = dp[i] + ret ;
    }
    println!("{}", dp[N]);
}