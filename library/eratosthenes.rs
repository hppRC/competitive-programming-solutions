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
        N: usize,
    }
    let era = Eratosthenes::new(N);

    let mut cnt = vec![0; N+1];
    let mut ans = 0;

    for i in 2..N+1 {
        let factors = era.factorization(i);
        for (k, v) in factors.into_iter() {
            cnt[k] += v;
        }
    }
    println!("{:?}", cnt.into_iter().fold(1, |acc, t| acc * (t + 1) % MOD));
;}

#[derive(Debug, Clone)]
struct Eratosthenes {
    end: usize,
    primes: Vec<usize>,
    flags: Vec<usize>,
}
impl Eratosthenes {
    fn new(n: usize) -> Self {
        let bits = 32;
        let flags_num = n / bits + 1;
        let defaults: Vec<usize> = vec![0x5D75D75D, 0xD75D75D7, 0x75D75D75];

        let (mut i, mut f, mut j);
        let max = ((n as f32).sqrt() as usize) + 1;

        let mut flags: Vec<usize> = (0..flags_num).map(|i| defaults[i % 3]).collect();
        flags[flags_num - 1] &= (1 << (n % bits + 1)) - 1 ;

        i = 5;
        f = 4;

        while i <= max {
            let t = (flags[i / bits] >> (i % bits)) & 1 == 1;
            if !t {
                j = i * (i | 1);
                while j <= n {
                    flags[j / bits] |= 1 << (j % bits);
                    j += i * 2;
                }
            }
            f = 6 - f;
            i += f;
        }

        //2, 3のフラグを消す
        flags[0] &= !0b1100;
        //1のフラグを立てる(素数から除く,のちの素数判定のため)
        flags[0] |= 0b11;

        let mut primes = vec![];

        for p in 2..n+1 {
            if (flags[p / bits] >> (p % bits)) & 1 == 0 {
                primes.push(p);
            }
        }

        Eratosthenes {
            end: n,
            primes: primes,
            flags: flags,
        }
    }
    #[allow(dead_code)]
    fn is_prime(&self, m: usize) -> bool {
        self.flags[m / 32] >> (m % 32) & 1 == 0
    }
    #[allow(dead_code)]
    fn factorization(&self, n: usize) -> HashMap<usize, usize> {
        let mut n = n;
        let mut factors: HashMap<usize, usize> = HashMap::new();
        for &p in &self.primes {
            while n % p == 0 {
                n /= p;
                *factors.entry(p).or_insert(0) += 1;
            }
            if p > n {
                break;
            }
        }
        factors
    }
}

