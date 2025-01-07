#![feature(cmp_minmax)]
use std::cmp;
use std::env;
use std::time::Instant;

const M: usize = 128;
static S: [usize; 7] = [0, 1, 3, 6, 10, 15, 21];


fn answer(n: usize) -> u128 {
    fn _idx(i: usize, j: usize, k: usize) -> usize {
        let [x, y] = cmp::minmax(i, j);
        (S[y] + x) * M + (k & M-1)
    }

    let coins: [usize; 7] = [1, 2, 6, 12, 24, 48, 60];
    let mut arr = [0u128; 28 * M];
    for k in (0..M).step_by(2) {
        arr[k] = 1;
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

fn main() {
    let arg = env::args().nth(1).expect("missing n");
    let n: usize = arg.parse().expect("malformed n");
    let t1 = Instant::now();
    let a = answer(n);
    let elapsed = Instant::now() - t1;
    println!("{}", a);
    println!("Elapsed: {}μs", elapsed.as_micros());
}
