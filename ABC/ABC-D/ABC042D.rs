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

#[allow(dead_code)]
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
#[allow(dead_code)]
fn lcm(a: u64, b:u64) -> u64 {
    a * b / gcd(a, b)
}
#[allow(dead_code)]
fn mod_pow(x: u64, n: u64, m: u64) -> u64 {
    let mut res = 1;
    let mut x = x;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = (res * x) % m;
        }
        x = (x * x) % m;
        n = n >> 1;
    }
    res
}
#[allow(dead_code)]
fn mod_inv(a: u64, m: u64) -> u64 {
    let (_, x, _) = extgcd(a as i64, m as i64);
    (m as i64 + x) as u64 % m
}
#[allow(dead_code)]
fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x, y) = extgcd(b, a % b);
        (gcd, y, x - (a / b) * y)
    }
}

#[derive(Debug)]
struct FactInv {
    fact: Vec<u64>,
    inv: Vec<u64>,
    factinv: Vec<u64>,
    m: u64,
}
impl FactInv {
    fn new(n: u64, m: u64) -> Self {
        let mut fact = vec![0; n as usize + 1];
        fact[0] = 1;
        for i in 1..n+1 {
            fact[i as usize] = i * &fact[i as usize - 1] % m;
        }
        let mut inv = vec![0; n as usize + 1];
        inv[0] = 0;
        inv[1] = 1;
        for i in 2..n+1 {
            inv[i as usize] = inv[(m % i) as usize] * (m - m / i) % m;
        }
        let mut factinv = vec![0; n as usize + 1];
        factinv[0] = 1;
        for i in 1..n+1 {
            factinv[i as usize] = factinv[i as usize - 1] * inv[i as usize] % m;
        }
        FactInv {
            fact: fact,
            inv: inv,
            factinv: factinv,
            m: m,
        }
    }
    fn comb(&self, n: u64, r: u64) -> u64 {
        if n < r {
            0
        } else {
            (self.fact[n as usize] * self.factinv[r as usize] % self.m) * self.factinv[(n-r) as usize] % self.m
        }
    }
}



fn main() {
    input! {
        H: u64, W: u64,
        A: u64, B: u64,
    }
    let fi = FactInv::new(H+W, MOD);
    let mut ans = 0;

    for x in B+1..W+1 {
        ans = (ans + fi.comb(H - A + x - 2, x - 1) * fi.comb(A - 1 + W - x, A - 1)) % MOD;
    }
    println!("{}", ans);
}

