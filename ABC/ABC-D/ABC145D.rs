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

#[derive(Eq, PartialEq, Clone, Debug)]
struct FactInv {
    fact: Vec<usize>,
    inv: Vec<usize>,
    factinv: Vec<usize>,
    m: usize,
}
#[allow(dead_code)]
impl FactInv {
    fn new(n: usize, m: usize) -> Self {
        let mut fact: Vec<usize> = vec![0; n + 1];
        fact[0] = 1;
        for i in 1..n+1 {
            fact[i] = i * &fact[i - 1] % m;
        }
        let mut inv = vec![0; n + 1];
        inv[0] = 0;
        inv[1] = 1;
        for i in 2..n+1 {
            inv[i] = inv[m % i] * (m - m / i) % m;
        }
        let mut factinv = vec![0; n + 1];
        factinv[0] = 1;
        for i in 1..n+1 {
            factinv[i] = factinv[i - 1] * inv[i] % m;
        }
        FactInv {
            fact: fact,
            inv: inv,
            factinv: factinv,
            m: m,
        }
    }
    fn comb(&self, n: usize, r: usize) -> usize {
        if n < r {
            0
        } else {
            (self.fact[n] * self.factinv[r] % self.m) * self.factinv[n-r] % self.m
        }
    }
}

fn main() {
    input!{
        X: usize, Y: usize,
    }

    let mut ans = 0;

    if (X + Y) % 3 != 0 {
        println!("{}",ans);
    } else if X * 2 < Y || X > Y * 2 {
        println!("{}", ans);
    } else {
        let n = (X + Y) / 3;
        let (X, Y) = if X > Y {(Y, X)} else {(X, Y)};

        let mut x = n;
        let mut y = 2 * n;
        let mut l = 0;
        let mut r = n;
        loop {
            if x == X && y == Y {
                break
            } else {
                x += 1;
                y -= 1;
                l += 1;
                r -= 1;
            }
        }
        let fi = FactInv::new(n, MOD);
        println!("{}", fi.comb(n, r));

    }

}