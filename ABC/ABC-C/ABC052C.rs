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

//thanks to https://qiita.com/Cassin01/items/2f90aedded2b8fb017a1
struct Eratosthenes {
    n: usize,
    primes: Vec<usize>,
    is_prime: Vec<bool>,
}

impl Eratosthenes {
    fn new(n: usize) -> Self {
        let mut spf = vec![None; n+1];
        let mut is_prime = vec![true; n+1];
        let mut primes = Vec::new();

        is_prime[0] = false;
        is_prime[1] = false;

        for i in 2..n+1 {
            if is_prime[i] {
                primes.push(i);
                spf[i] = Some(i);
            }
            for prime in &primes {
                if i * prime >= n + 1 || prime > &spf[i].unwrap() {
                    break;
                }
                is_prime[i * prime] = false;
                spf[i * prime] = Some(*prime);
            }
        }
        Eratosthenes {
            n: n,
            primes: primes,
            is_prime: is_prime
        }
    }
}




fn main() {
    input!{
        N: usize,
    }
    let mut cnt = vec![1; N+1];

    for i in 2..N+1 {
        let mut k = i;
        let mut flag = true;
        while flag {
            flag = false;
            for j in 2..i+1 {
                if k % j == 0 {
                    k /= j;
                    cnt[j] += 1;
                    flag = true;
                    break;
                }
            }
        }
    }
    println!("{}", cnt.into_iter().fold(1, |acc, x| acc * x % MOD));

}