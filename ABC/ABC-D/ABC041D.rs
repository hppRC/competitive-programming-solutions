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

fn main() {
    input!{
        N: usize, M:usize,
        xy: [(usize1, usize1); M]
    }

    let mut prev: Vec<usize> = vec![0; N];
    for (x, y) in xy {
        //頂点xからyへの有向辺(xのほうがyより先にゴール)
        prev[x] |= 1 << y;
    }

    let mut dp: Vec<usize> = vec![0; 1<<N];
    dp[0] = 1;


    for S in 1..1<<N {
        for v in 0..N {
            //頂点vがSに含まれていなければ無視
            if S & (1 << v) == 0 { continue }
            //頂点vからの出次数がSにおいて0ならば,再帰的にトポロジカルソートを行える
            if (prev[v] & S) == 0 {
                let rmv = !(1 << v);
                dp[S] += dp[S & rmv];
            }
        }
    }

    println!("{}", dp[(1<<N)-1]);

}