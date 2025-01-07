use std::env;
use std::time::Instant;

fn answer(n: usize) -> u128 {
    fn _idx(i: usize, j: usize, k: usize) -> usize {
        896 * i + 128 * j + k % 128
    }

    let coins: [usize; 7] = [1, 2, 6, 12, 24, 48, 60];
    let mut array = [0u128; 6272];
    for k in (0..128).step_by(2) {
        array[k] = 1;
    }

    for s in (0..n+1).step_by(128) {
        for i in 1..coins.len() {
            let icoin = coins[i];
            for j in 0..i + 1 {
                let jcoin = coins[j];
                let z = icoin + jcoin;
                for _k in 0..128 {
                    let k = s + _k;
                    let mut q: u128;
                    if j == 0 {
                        q = array[_idx(i-1, j, k)];
                    } else {
                        let left = array[_idx(i, j-1, k)];
                        let top = array[_idx(i-1, j, k)];
                        let diag = array[_idx(i-1, j-1, k)];
                        q = left + top - diag;
                    }
                    if z <= k {
                        q += array[_idx(i, j, k-z)];
                    }
                    array[_idx(i, j, k)] = q;
                    array[_idx(j, i, k)] = q;
                }
            }
        }    
    }

    array[_idx(6, 6, n)]
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
