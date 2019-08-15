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
        N: usize, M: usize,
        abc: [(usize1, usize1, usize); M],
    }
    //最短距離を格納
    let mut d = vec![vec![MOD; N]; N];
    //next[i][j] := i,j間の最短経路におけるiの次の点
    let mut next = vec![vec![0; N]; N];

    //辺の重みを入れる
    for (a, b, c) in abc {
        d[a][b] = c;
        d[b][a] = c;
    }

    //自己ループは重み0(今回はないけど)
    for i in 0..N {
        d[i][i] = 0;
    }

    //初期化しておくiからjまでの最短経路でのiの次の点をとりあえずjで初期化
    for i in 0..N {
        for j in 0..N {
            next[i][j] = j;
        }
    }

    //ワーシャルフロイド
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                if d[i][j] > d[i][k] + d[k][j] {
                    d[i][j] = d[i][k] + d[k][j];
                    next[i][j] = next[i][k];
                }
            }
        }
    }

    //任意のiからjまでの最短経路に出てくる辺を入れる
    let mut edges = HashSet::new();

    for i in 0..N {
        for j in i+1..N {
            let mut to = next[i][j];
            let mut prev = i;
            while to != j {
                if !(edges.contains(&(prev, to)) || edges.contains(&(to, prev))) {
                    edges.insert((prev, to));
                }
                prev = to;
                to = next[to][j];
            }
            if !(edges.contains(&(prev, to)) || edges.contains(&(to, prev))) {
                edges.insert((prev, to));
            }
        }
    }
    println!("{}", M - edges.iter().fold(0, |acc, _| acc + 1));
}