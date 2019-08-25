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
trait CharUtils {
    fn to_num(self) -> usize;
}
impl CharUtils for char {
    fn to_num(self) -> usize {
        self as usize
    }
}
trait UsizeUtils {
    fn sqrt_floor(self) -> usize;
    fn sqrt_ceil(self) -> usize;
    fn factors(self) -> HashMap<usize, usize>;
    fn digits(self) -> usize;
    fn bit_length(self) -> usize;
}
impl UsizeUtils for usize {
    fn sqrt_floor(self) -> usize {
        (self as f64).sqrt() as usize
    }
    fn sqrt_ceil(self) -> usize {
        let tmp = (self as f64).sqrt() as usize;
        if tmp * tmp == self {
            tmp
        } else {
            tmp + 1
        }
    }
    fn factors(self) -> HashMap<usize, usize> {
        let mut facs: HashMap<usize, usize> = HashMap::new();
        let mut n = self;
        let mut a = 2;
        while n >= a*a {
            if n % a == 0{
                n /= a;
                *facs.entry(a).or_insert(0) += 1;
            } else {
                a += 1;
            }
        }
        *facs.entry(n).or_insert(0) += 1;
        facs
    }
    fn digits(self) -> usize {
        (self as f64).log10() as usize + 1
    }
    fn bit_length(self) -> usize {
        let mut ret = 0;
        let mut n = self;
        while n > 0 {
            ret += 1;
            n >>= 1;
        }
        ret
    }
}
trait VecUtils<T> {
    fn transpose(&self) -> Vec<Vec<&T>>;
}
impl<T> VecUtils<T> for Vec<Vec<T>> {
    fn transpose(&self) -> Vec<Vec<&T>> {
        (0..self[0].len()).map(|j|  (0..self.len()).map(|i| &self[i][j]).collect::<Vec<&T>>()).collect::<Vec<Vec<&T>>>()
    }
}


fn main() {
    input!{
        N: usize,
    }
    let bit = N.bit_length();
    let mut v = vec![vec![0; N]; N];

    for i in 0..N {
        for j in i+1..N {
            let k = i ^ j;
            for l in 0..bit {
                if (k >> l) & 1 == 1 {
                    v[i][j] = l + 1;
                    break;
                }
            }
        }
    }

    for i in 0..N-1 {
        for j in i+1..N {
            print!("{} ", v[i][j]);
        }
        println!("");
    }
}