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
macro_rules! printvec {
    ( $item:expr ) => {
        for &i in &$item {
            print!("{} ", i);
        }
        println!("");
    }
}
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Rev<T>(pub T);
impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}
impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct Total<T>(pub T);
impl<T: PartialEq> Eq for Total<T> {}
impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
#[allow(dead_code)]
const MOD: usize = 1000000007;
#[allow(dead_code)]
fn factorial(n: usize) -> usize {
    (1..n+1).into_iter().fold(1, |acc, i| acc * i)
}
#[allow(dead_code)]
fn comb(n: usize, r: usize) -> usize {
    if n - r < r {
        comb(n, n - r)
    } else {
        (1..r+1).into_iter().fold(1, |acc, i| acc * (n - r + i) / i)
    }
}

fn mod_pow(x: usize, n: usize, m: usize) -> usize {
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

fn main() {
    input!{
        N: usize, K: usize,
        A: [usize; N],
    }

    let mut sum = 0;
    let mut hash = HashMap::new();

    for &a in &A {
        *hash.entry(a).or_insert(0) += 1;
        sum += a;
    }

    let mut B: Vec<(usize, usize)> = vec![];
    for (k, v) in hash {
        B.push((k, v));
    }
    B.sort();

    let mut dp = vec![0; sum * 2 + 1];
    dp[0] = 1;

    for i in 0..B.len() {
        let mut next = vec![0; sum * 2 + 1];
        for j in 0..sum {
            next[j ^ B[i].0] += dp[j] * mod_pow(2, B[i].1 - 1, MOD) % MOD;
            next[j] += dp[j] * mod_pow(2, B[i].1 - 1, MOD) % MOD;
        }
        dp = next;
    }

    println!("{}", dp[K]);
}


/*
fn main() {
    input!{
        N: usize, K: usize,
        a: [usize; N],
    }
    let mut a = a;
    a.sort();

    let mut dp = vec![HashMap::new(); N+1];
    dp[0].insert(0, 1);

    let mut limit = 0;

    for i in 0..N {
        for j in 0..limit+1 {
            *dp[i+1].entry(j).or_insert(0) += *dp[i].get(&j).unwrap_or(&0);
            *dp[i+1].entry(j).or_insert(0) %= MOD;
            *dp[i+1].entry(j^a[i]).or_insert(0) += *dp[i].get(&j).unwrap_or(&0);
            *dp[i+1].entry(j^a[i]).or_insert(0) %= MOD;
        }
        limit |= a[i];
    }
    println!("{}", dp[N].get(&K).unwrap_or(&0));
}
*/