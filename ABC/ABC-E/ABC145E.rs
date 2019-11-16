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
        N: usize, T: usize,
        AB: [(usize, usize); N],
    }

    let A: Vec<usize> = AB.iter().map(|&(a, _)| a).collect();
    let B: Vec<usize> = AB.iter().map(|&(_, b)| b).collect();
    //制限時間Tまでのやつ
    let mut dp1 = vec![vec![0; T+1]; N+1];

    for i in 0..N {
        for t in 0..T+1 {
            if t >= A[i] {
                dp1[i+1][t] = max(dp1[i][t], dp1[i][t-A[i]]+B[i]);
            } else {
                dp1[i+1][t] = dp1[i][t];
            }
        }
    }

    let mut dp2 = vec![vec![0; T]; N+1];

    for i in 0..N {
        for t in 0..T {
            if t >= A[i] {
                dp2[i+1][t] = max(dp2[i][t], dp2[i][t-A[i]]+B[i]);
            } else {
                dp2[i+1][t] = dp2[i][t];
            }
        }
    }
    let mut now = T-1;
    let mut used = vec![];
    for i in (1..N+1).rev() {
        if dp2[i][now] == dp2[i-1][now] {
            continue
        } else {
            now -= A[i-1];
            used.push(i-1);
        }
    }

    if used.len() < N {
        let mut last = 0;
        for i in 0..N {
            if !used.contains(&i) {
                last = i;
                break
            }
        }

        for i in 0..N {
            if used.contains(&i) {
                continue
            } else {
                if B[last] < B[i] {
                    last = i;
                }
            }
        }

        println!("{}", max(dp1[N][T], dp2[N][T-1] + B[last]));
    } else {
        println!("{}", dp1[N][T])
    }

}