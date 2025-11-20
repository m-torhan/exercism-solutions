pub fn factors(n: u64) -> Vec<u64> {
    let mut ret: Vec<u64> = Vec::new();

    let sqrt_n: u64 = ((n as f64).sqrt() + 1.) as u64;

    let mut sieve  = vec![true; (sqrt_n + 2) as usize];

    for p in 2..sqrt_n + 1{
        if !sieve[p as usize] {
            continue;
        }
        let mut q = p + p;
        while q <= sqrt_n {
            sieve[q as usize] = false;
            q += p;
        }
    }

    let mut primes: Vec<u64> = Vec::new();

    for p in 2..sqrt_n + 1{
        if sieve[p as usize] {
            primes.push(p);
        }
    }

    let mut k = n;
    for p in primes {
        while k % p == 0 {
            ret.push(p);
            k /= p;
        }
    }
    if k != 1 {
        ret.push(k);
    }
    println!("n={n}");
    return ret;
}
