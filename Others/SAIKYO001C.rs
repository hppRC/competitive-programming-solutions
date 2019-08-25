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
    (1..n+1).into_iter().fold(1, |acc, i| acc * i % MOD)
}
#[allow(dead_code)]
fn comb(n: usize, r: usize) -> usize {
    if n - r < r {
        comb(n, n - r)
    } else {
        (1..r+1).into_iter().fold(1, |acc, i| acc * (n - r + i) / i)
    }
}

fn main() {
    input!{
        N: usize,
        S: chars,
    }

    let mut ans: isize = (factorial(N) % MOD) as isize;
    let mut lstack: isize = 0;

    for i in 0..2*N {
        if (lstack % 2 == 1 && S[i] == 'B') || (lstack % 2 == 0 && S[i] == 'W') {
            ans *= lstack;
            ans %= MOD as isize;
            lstack -= 1;
        } else {
            lstack += 1;
        }
    }

    if lstack == 0 {
        println!("{}", ans);
    } else {
        println!("0");
    }


/*
    if S[0] == 'W' || S[2*N-1] == 'W' {
        println!("0");
    } else {
        let mut lr = vec![true; 2*N];
        let mut l = 1;
        let mut r = 0;

        for i in 0..2*N-1 {
            if S[i] == S[i+1] {
                lr[i+1] = !lr[i];
            } else {
                lr[i+1] = lr[i];
            }
            if lr[i+1] {
                l += 1;
            } else {
                r += 1;
            }
        }

        if l == r {
            let mut ans = 1;
            let mut lstack = 0;
            for i in 0..2*N {
                if lr[i] {
                    lstack += 1;
                } else {
                    ans *= lstack % MOD;
                    ans %= MOD;
                    lstack -= 1;
                }
            }

            println!("{}", ans * factorial(N) % MOD);


        } else {
            println!("0");
        }
    }

*/

}