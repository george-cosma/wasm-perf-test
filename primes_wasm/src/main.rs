pub fn main() {
    let num_primes: u64 = std::env::args().collect::<Vec<_>>()[1].parse().unwrap();

    dbg!(primes(num_primes));
}

#[no_mangle]
pub fn primes(max_num: u64) -> u64 {
    let mut result = 0;
    for num in 0..=max_num {
        let mut prime = true;
        for k in 2..num {
            if num % k == 0 {
                prime = false;
                break;
            }
        }

        if prime {
            result += 1;
        }
    }

    return result;
}