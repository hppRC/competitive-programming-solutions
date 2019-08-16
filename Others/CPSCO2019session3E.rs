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
        N: usize,
        A: [usize; N],
    }
    //最大桁数(A < 2**30なので)
    const DIGIT: usize = 30;
    //j桁目の1の個数を格納
    let mut cnt = vec![0; DIGIT];

    //累積xor
    let mut X = 0;

    for i in 0..N {
        //各iごとにAと今までの累積xorをとる
        X ^= A[i];
        let mut res = 0;
        for j in 0..DIGIT {
            //A[i]のj桁目が1ならcnt[j]を1増やす
            if (A[i] >> j) & 1 == 1 {
                cnt[j] += 1;
            }
            //累積xorのj桁目が1なら
            if (X >> j) & 1 == 1 {
                //各A[0..i]のj桁目は1とのxorをとることになる
                //なので,今までのj桁目に存在する0の個数だけ,A[0..i]においてj桁目に1が立つ
                //j桁目だけを考えてあげれば, A[0..i] xor X の結果はj桁目に0が立っているものの総数 * 2をj-1乗(1 << j)したものになる
                //また,cnt[j]には今までのj桁目の1の個数が入っているので, 0の個数は i - cnt[j] で求まる. よって以下のようになる
                res += (i + 1 - cnt[j]) * (1 << j);
            } else {
                //累積xorのj桁目が0なら
                //各A[0..i]のj桁目は0とのxorをとることになる, つまりそのまま
                //上記同じように足してあげる
                res += cnt[j] * (1 << j);
            }
        }
        println!("{}", res);
    }
}