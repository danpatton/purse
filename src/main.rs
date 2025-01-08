#![feature(cmp_minmax)]
use core::fmt::Display;
use std::cmp;
use std::env;
use std::time::{Duration, Instant};

use bnum::BUint;
use num::Num;

type U192 = BUint<3>;

const M: usize = 128;
static S: [usize; 7] = [0, 1, 3, 6, 10, 15, 21];


fn answer<T: Num + Copy>(n: usize) -> T {
    fn _idx(i: usize, j: usize, k: usize) -> usize {
        let [x, y] = cmp::minmax(i, j);
        (S[y] + x) * M + (k & M-1)
    }

    let coins: [usize; 7] = [1, 2, 6, 12, 24, 48, 60];
    let mut arr = [T::zero(); 28 * M];
    for k in (0..M).step_by(2) {
        arr[k] = T::one();
    }

    for s in (0..n+1).step_by(M) {
        for i in 1..7 {
            let icoin = coins[i];
            for k in s..M+s {
                let z = icoin + 1;
                arr[_idx(i, 0, k)] = arr[_idx(i-1, 0, k)] + arr[_idx(i, 0, M+k-z)];
            }
            for j in 1..i + 1 {
                let jcoin = coins[j];
                let z = icoin + jcoin;
                for k in s..M+s {
                    let left = arr[_idx(i, j-1, k)];
                    let top = arr[_idx(i-1, j, k)];
                    let diag = arr[_idx(i-1, j-1, k)];
                    let back = arr[_idx(i, j, M+k-z)];
                    arr[_idx(i, j, k)] = left + top - diag + back;
                }
            }
        }
    }

    arr[_idx(6, 6, n)]
}

fn print_result<TResult: Display>(result: TResult, dur: Duration) {
    println!("{}", result);
    if dur.as_micros() >= 1000 {
        println!("Elapsed: {:.1}ms", dur.as_secs_f64() * 1e3);
    } else {
        println!("Elapsed: {}μs", dur.as_micros());
    }
}

fn main() {
    let arg = env::args().nth(1).expect("missing n");
    let n: usize = arg.parse().expect("malformed n");
    if n > 4800 {
        let t1 = Instant::now();
        let a: U192 = answer(n);
        print_result(a, Instant::now() - t1);
    } else {
        let t1 = Instant::now();
        let a: u128 = answer(n);
        print_result(a, Instant::now() - t1);
    }
}
