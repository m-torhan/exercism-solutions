pub fn nth(n: u32) -> u32 {
    let mut prime_count = (((u32::MAX as f32)/(u32::MAX as f32).ln()) + 1.) as u32;
    if n + 1 < ((prime_count as f32).sqrt() as u32) {
        prime_count = (n + 1)*(n + 1) + 2;
    }
    let mut sieve = vec![true; prime_count as usize];
    let prime_count_sqrt = ((prime_count as f32).sqrt() + 1.) as u32;

    for p in 2..prime_count_sqrt {
        if !sieve[p as usize] {
            continue;
        }
        let mut q = p << 1;
        while q < prime_count {
            sieve[q as usize] = false;
            q += p;
        }
    }

    let mut k = (n + 1) as i32;
    let mut i: usize = 0;
    while k >= 0 {
        i += 1;
        if sieve[i] {
            k -= 1;
        }
    }

    return i as u32;
}
