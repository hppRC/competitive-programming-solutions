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
const MOD: usize = 998244353;
#[allow(dead_code)]
fn to_num(c: char) -> i64 {
    c as i64 - 48
}

#[derive(Debug)]
struct FactInv {
    fact: Vec<u64>,
    inv: Vec<u64>,
    factinv: Vec<u64>,
    m: u64,
}
#[allow(dead_code)]
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
    input!{
        N: usize,
        S: chars,
    }
    let fi = FactInv::new(N as u64, MOD as u64);

    let mut red = vec![];
    let mut green = vec![];
    let mut blue = vec![];

    for j in 0..3*N {
        if S[j] == 'R' {
            red.push(j);
        } else if S[j] == 'G' {
            green.push(j);
        } else {
            blue.push(j);
        }
    }

    let mut ans = fi.fact[N];

    let mut i = 0;
    let mut j = 1;

    while j < N {
        if red[j] < green[i] && green[j] < blue[i] {
        } else {
            ans = ans * (fi.fact[j - i].pow(2)) % MOD as u64;
            i = j;
        }
        j += 1;
    }
    ans = ans * (fi.fact[j - i].pow(2)) % MOD as u64;

    i = 0;
    j = 1;
    while j < N {
        if red[j] < blue[i] && blue[j] < green[i] {
        } else {
            ans = ans * (fi.fact[j - i].pow(2)) % MOD as u64;
            i = j;
        }
        j += 1;
    }
    ans = ans * (fi.fact[j - i].pow(2)) % MOD as u64;


    i = 0;
    j = 1;
    while j < N {
        if blue[j] < red[i] && red[j] < green[i] {
        } else {
            ans = ans * (fi.fact[j - i].pow(2)) % MOD as u64;
            i = j;
        }
        j += 1;
    }
    ans = ans * (fi.fact[j - i].pow(2)) % MOD as u64;


    i = 0;
    j = 1;
    while j < N {
        if blue[j] < green[i] && green[j] < red[i] {
        } else {
            ans = ans * (fi.fact[j - i].pow(2)) % MOD as u64;
            i = j;
        }
        j += 1;
    }
    ans = ans * (fi.fact[j - i].pow(2)) % MOD as u64;


    i = 0;
    j = 1;
    while j < N {
        if green[j] < red[i] && red[j] < blue[i] {
        } else {
            ans = ans * (fi.fact[j - i].pow(2)) % MOD as u64;
            i = j;
        }
        j += 1;
    }
    ans = ans * (fi.fact[j - i].pow(2)) % MOD as u64;


    i = 0;
    j = 1;
    while j < N {
        if green[j] < blue[i] && blue[j] < red[i] {
        } else {
            ans = ans * (fi.fact[j - i].pow(2)) % MOD as u64;
            i = j;
        }
        j += 1;
    }
    ans = ans * (fi.fact[j - i].pow(2)) % MOD as u64;

    println!("{}", ans);

}