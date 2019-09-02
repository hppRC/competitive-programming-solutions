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


fn main() {
    input!{
        N: usize,
        xy: [(isize, isize); N],
    }
    let mut dir: Vec<(isize, isize)> = vec![(0,0); 8];

    for &(x, y) in &xy {
        if x == 0 {
            if y == 0 {
                continue;
            } else if y > 0 {
                dir[0] = (dir[0].0 + x, dir[0].1 + y);
            } else {
                dir[4] = (dir[4].0 + x, dir[4].1 + y);
            }
        } else if y == 0 {
            if x == 0 {
                continue;
            } else if x > 0 {
                dir[2] = (dir[2].0 + x, dir[2].1 + y);
            } else {
                dir[6] = (dir[6].0 + x, dir[6].1 + y);
            }
        } else {
            if x > 0 && y > 0 {
                dir[1] = (dir[1].0 + x, dir[1].1 + y);
            } else if x > 0 && y < 0 {
                dir[3] = (dir[3].0 + x, dir[3].1 + y);
            } else if x < 0 && y < 0 {
                dir[5] = (dir[5].0 + x, dir[5].1 + y);
            } else {
                dir[7] = (dir[7].0 + x, dir[7].1 + y);
            }
        }
    }

    let mut ans = 0;

    for i in 0..(1<<8) {
        let mut tmp = (0, 0);
        let mut bit = i;
        for j in 0..8 {
            if (bit >> j) & 1 == 1 {
                tmp = (tmp.0 + dir[j].0, tmp.1 + dir[j].1);
            }
        }
        ans = max(ans, tmp.0*tmp.0 + tmp.1*tmp.1);
    }
    println!("{}", (ans as f64).sqrt());
}