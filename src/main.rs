use std::env;

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
                        let top = array[_idx(i-1, j, k)];
                        // println!("i={}, j={}, k={}, top={}", i, j, k, top);
                        q = top;
                    } else {
                        let left = array[_idx(i, j-1, k)];
                        let top = array[_idx(i-1, j, k)];
                        let diag = array[_idx(i-1, j-1, k)];
                        // println!("i={}, j={}, k={}, left={}, top={}, diag={}", i, j, k, left, top, diag);
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
    let args: Vec<String> = env::args().collect();
    let n: usize = args.get(1).expect("missing n").parse().expect("malformed n");
    let a = answer(n);
    println!("{}", a);
}
